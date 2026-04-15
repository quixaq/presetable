use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Preset { id: Option<usize> },
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
