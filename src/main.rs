/*
 *     presetable v0.1.1
 * Copyright (C) 2026  Quixaq
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use duct::cmd;
use inquire::Select;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::{fmt, fs, process};
use toml;

#[derive(Parser)]
#[command(name = "presetable", about = "A tool for command alias presets")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Change the active preset
    Preset {
        /// The preset you want to choose. If not provided, an interactive picker will open.
        id: Option<usize>,
    },
    /// Run a command from the preset
    Run {
        /// The command you want to run. If not provided, an interactive picker will open.
        id: Option<usize>,
    },
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct State {
    pub preset: usize,
}

impl State {
    fn get_path() -> PathBuf {
        let project_dirs = ProjectDirs::from("com", "quixaq", "presetable")
            .expect("Could not find valid home directory");
        project_dirs.data_dir().join("state.toml")
    }

    pub fn load() -> Self {
        let path = Self::get_path();
        fs::read_to_string(path)
            .ok()
            .and_then(|contents| toml::from_str(&contents).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::get_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let serialized = toml::to_string(self).expect("Failed to serialize state");
        fs::write(path, serialized)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Preset {
    pub name: String,
    pub values: BTreeMap<usize, String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub presets: HashMap<usize, Preset>,
}

impl Config {
    pub fn load() -> Self {
        let project_dirs = ProjectDirs::from("com", "quixaq", "presetable")
            .expect("Could not find valid home directory");
        let path = project_dirs.config_dir().join("config.toml");
        if !path.exists() {
            eprintln!("Error: Config not found at {:?}", path);
            process::exit(1);
        }

        let content = fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Error: Could not read config: {}", e);
            process::exit(1);
        });

        toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Error: Malformed config file: {}", e);
            process::exit(1);
        })
    }
}

struct PresetOption<'a> {
    id: &'a usize,
    name: &'a String,
}
impl<'a> fmt::Display for PresetOption<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn preset(id: Option<usize>, config: Config) {
    match id {
        Some(id) => {
            if !config.presets.contains_key(&id) {
                eprintln!("Error: Provided preset doesn't exist");
                process::exit(1);
            }
            State::save(&State { preset: id }).unwrap_or_else(|e| {
                eprintln!("Error: Failed to save state: {}", e);
                process::exit(1);
            })
        }
        None => {
            let presets: Vec<PresetOption> = config
                .presets
                .iter()
                .map(|(id, p)| PresetOption { id, name: &p.name })
                .collect();

            if presets.is_empty() {
                eprintln!("Error: No presets defined in config");
                process::exit(1);
            }

            match Select::new("Choose preset:", presets).prompt() {
                Ok(choice) => State::save(&State { preset: *choice.id }).unwrap_or_else(|e| {
                    eprintln!("Error: Failed to save state: {}", e);
                    process::exit(1)
                }),
                Err(e) => {
                    eprintln!("Error: Failed to prompt for config: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}

struct CommandOption<'a> {
    id: &'a usize,
    cmd: &'a String,
}
impl<'a> fmt::Display for CommandOption<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.id, self.cmd)
    }
}

fn run(id: Option<usize>, config: Config) {
    let preset = &State::load().preset;
    let final_id = if let Some(id) = id {
        id
    } else {
        let options: Vec<CommandOption> = config
            .presets
            .get(preset)
            .map(|p| {
                p.values
                    .iter()
                    .map(|(id, cmd)| CommandOption { id, cmd })
                    .collect()
            })
            .unwrap_or_else(|| {
                eprintln!("Error: Failed to read commands for preset");
                process::exit(1)
            });

        match Select::new("Pick command to run:", options).prompt() {
            Ok(id) => *id.id,
            Err(e) => {
                eprintln!("Error: Failed to prompt for config: {}", e);
                process::exit(1);
            }
        }
    };
    if let Some(command) = config
        .presets
        .get(preset)
        .and_then(|p| p.values.get(&final_id))
    {
        let (shell, flag) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        if let Err(e) = cmd!(shell, flag, command).run() {
            eprintln!("Error: Failed to execute command: {}", e);
            process::exit(1);
        }
    } else {
        eprintln!("Error: Could not find command in preset");
        process::exit(1);
    }
}

fn main() {
    let cli = Cli::parse();
    let config = Config::load();

    match &cli.command {
        Commands::Preset { id } => preset(*id, config),
        Commands::Run { id } => run(*id, config),
    }
}
