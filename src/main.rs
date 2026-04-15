use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Preset { id: Option<usize> },
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Preset { id } => match id {
            Some(_id) => todo!("Choose the provided preset"),
            None => todo!("Interactive prompt for preset"),
        },
    }
}
