use std::env::current_dir;

use clap::{Parser, Subcommand};

pub mod notebook;

use notebook::{ Notebook, page::Page };

use crate::notebook::chapter::Chapter;

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
    },

    /// Create new components in the notebook
    New {
        #[command(subcommand)]
        subcommand: NewCommand
    }
        
}

#[derive(Subcommand)]
enum NewCommand {
    Page { name: String },
    Chapter { title: String }
}

fn main() {
    let cli = Cli::parse();
    let current_dir = match current_dir() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Unable to access current directory with error {err}");
            std::process::exit(-1);
        }
    };

    match &cli.command {
        Commands::Init { name } => { 
            let _notebook = Notebook::init_notebook(current_dir, name.to_owned());
        }
        Commands::New { subcommand } => match subcommand {
            NewCommand::Page{ name } => { 
                let _new_page = Page::create_page(current_dir, name.to_owned());
            },
            NewCommand::Chapter { title } => {
                let _new_chapter = Chapter::create_chapter(current_dir, title.to_owned());
            }
        }
    };
}
