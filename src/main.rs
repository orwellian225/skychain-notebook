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
    /// View the contents of the notebook
    View {
        #[command(subcommand)]
        subcommand: ViewCommand
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

#[derive(Subcommand)]
enum ViewCommand {
    /// View a summary of the selected component
    Summary { 
        #[command(subcommand)]
        selection: ViewSummarySelection
    },
    /// View all the notebooks information - Chapters, Pages, Cells
    Notebook,
    /// View the selected chapter's information - Pages 
    Chapter { identifier: String },
    /// View the selected pages's information - Cells
    Page { identifier: String },
    /// View the selected cell's information
    Cell { identifier: String },
}

#[derive(Subcommand)]
enum ViewSummarySelection  {
    /// View a summary of the notebook
    Notebook,
    /// View a summary of the selected chapter
    Chapter { identifier: String },
    /// View a summary of the selected page
    Page { identifier: String }
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
            },
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
                notebook.save_notebook();
            },
            Commands::View { subcommand} => {
                let notebook = Notebook::load_notebook(&current_dir);
                match subcommand {
                    ViewCommand::Notebook => { println!("View Notebook Information"); todo!(); }
                    ViewCommand::Chapter { identifier  } => { println!("View Chapter {identifier} Information"); todo!(); }
                    ViewCommand::Page { identifier  } => { println!("View Page {identifier} Information"); todo!(); }
                    ViewCommand::Cell { identifier  } => { println!("View Cell {identifier} Information"); todo!(); }
                    ViewCommand::Summary { selection } => match selection {
                        ViewSummarySelection::Notebook => { println!("View Notebook summary"); todo!(); }
                        ViewSummarySelection::Chapter { identifier } => { println!("View chapter {identifier} summary"); todo!(); }
                        ViewSummarySelection::Page { identifier } => { println!("View page {identifier} summary"); todo!(); }
                    }
                }
            }
        }
    };
}
