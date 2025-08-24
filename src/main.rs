use std::env::current_dir;

use clap::{Parser, Subcommand};

pub mod notebook;

use notebook::{ Notebook, page::Page };

use crate::notebook::chapter::Chapter;

#[derive(Parser)]
#[command(version, about)]
struct SkychainCli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new skychain notebook.
    Init { title: Option<String> },
    /// Add components to the notebook
    Add {
        #[command(subcommand)]
        subcommand: AddCommand
    },
    /// Modify the config of the notebook
    Config {
        #[arg(short, long)]
        global: bool
    }
}

#[derive(Subcommand)]
enum AddCommand {
    Page { title: String },
    Chapter { title: String }
}

fn main() {
    let cli = SkychainCli::parse();
    let current_dir = match current_dir() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Unable to access current directory with error {err}");
            std::process::exit(-1);
        }
    };

    match &cli.command {
        None => { todo!(); }
        Some(cmd) => match cmd {
            Commands::Init { title: name } => { 
                let _notebook = Notebook::init_notebook(&current_dir, name.to_owned());
            },
            Commands::Config { global } => {
                todo!();
            }
            Commands::Add { subcommand } => { 
                let mut notebook = Notebook::load_notebook(&current_dir);
                match subcommand {
                    AddCommand::Page{ title: name } => { 
                        let new_page = Page::create_page(current_dir, name.to_owned());
                        notebook.pages_mut().push(new_page);
                    },
                    AddCommand::Chapter { title } => {
                        let new_chapter = Chapter::create_chapter(current_dir, title.to_owned());
                        notebook.chapters_mut().push(new_chapter);
                    }
                }
            }
        }
    };
}
