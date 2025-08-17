use std::env::current_dir;

use clap::{Parser, Subcommand};

pub mod notebook;
use notebook::Notebook;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new skychain notebook.
    Init {
        name: Option<String>,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { name } => Notebook::init_notebook(current_dir().unwrap(), name.to_owned())
    };
}
