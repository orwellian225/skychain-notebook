//! skychain notebook

use std::fmt::Debug;
use std::io::Write;
use std::path::PathBuf;
use std::fs::{read, create_dir, read_dir, File};
use std::process::exit;

use serde::{Serialize, Deserialize};
use toml;

pub mod chapter;
pub mod page;
pub mod cell;

use chapter::Chapter;
use page::Page;
use cell::Cell;
use cell::types::MarkdownCell;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Author {
    pub name: String,
    pub organization: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notebook {
    /// Identifier is the (unique across the notebook directory) identifier of the notebook
    identifier: String,

    /// The non-unique title of the notebook
    title: String,
    authors: Vec<Author>,
    cells: Vec<Box<dyn Cell>>,

    chapters: Vec<Chapter>,
    pages: Vec<Page>
}

impl Default for Notebook {
    fn default() -> Self {
        let first_cell_content = format!("This is a Skychain Notebook!\n\nWe use algorithms and some clever thinking to improve the notebook experience, but the sky is lower then jupiter, so please lower your expectations.");

        Notebook { 
            identifier: "new_notebook".to_string(),
            title: "New Notebook".to_string(), 
            authors: vec![ Author { name: "New Author".to_string(), ..Default::default() } ],
            cells: vec![
	            Box::new(MarkdownCell::create_cell(first_cell_content))
            ], 
            chapters: Vec::new(), 
            pages: Vec::new()
        }
    }
}

impl Notebook {
    pub fn title(&self) -> &String { &self.title }
    pub fn authors(&self) -> &Vec<Author> { &self.authors }
    pub fn cells(&self) -> &Vec<Box<dyn Cell>> { &self.cells }
    pub fn chapters(&self) -> &Vec<Chapter> { &self.chapters }
    pub fn pages(&self) -> &Vec<Page> { &self.pages }

    pub fn title_mut(&mut self) -> &mut String { &mut self.title }
    pub fn authors_mut(&mut self) -> &mut Vec<Author> { &mut self.authors }
    pub fn cells_mut(&mut self) -> &mut Vec<Box<dyn Cell>> { &mut self.cells }
    pub fn chapters_mut(&mut self) -> &mut Vec<Chapter> { &mut self.chapters }
    pub fn pages_mut(&mut self) -> &mut Vec<Page> { &mut self.pages }
}

impl Notebook {
    pub fn init_notebook(current_directory: PathBuf, project_name: Option<String>) -> Notebook {
        let notebook_dir= match project_name {
            Some(val) => current_directory.join(val),
            None => current_directory.clone()
        };

        let project_identifier= match &notebook_dir.file_name() {
            Some(val) => match val.to_str() {
                Some(val) => val.to_string(),
                None => {
                    eprintln!("Project directory {} is not valid unicode", &notebook_dir.as_os_str().to_str().unwrap());
                    exit(-1);
                }
            },
            None => {
                eprintln!("Project directory {} cannot be used to identify a project name", &notebook_dir.as_os_str().to_str().unwrap());
                exit(-1);
            }
        };

        let project_title = project_identifier.replace("_", " ");
        let notebook = Notebook { 
            identifier: project_identifier, 
            title: project_title,
            ..Default::default()
        };

        println!("Creating notebook \"{}\" in {}", &notebook.identifier, &notebook_dir.to_str().unwrap());
        if !notebook_dir.exists() {
            match create_dir(&notebook_dir) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("Failed to create the notebook directory with error: {err}");
                    exit(-1)
                }
            };
        };

        let serialized_notebook = match toml::to_string_pretty(&notebook) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Failed to convert notebook string with error: {err}");
                exit(-1);
            }
        };

        let notebook_filepath = notebook_dir.join("main.iscnb");
        let mut notebook_file = match File::create(notebook_filepath) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Failed to create notebook file with error: {err}");
                exit(-1);
            }
        };
        match notebook_file.write_all(serialized_notebook.as_bytes()) {
            Ok(_) => {},
            Err(err) => {
                eprintln!("Failed to write notebook data to file with error: {err}");
                exit(-1);
            }
        };

        notebook
    }

    pub fn load_notebook(current_directory: &PathBuf) -> Notebook {
        let notebook_filepaths: Vec<PathBuf> = read_dir(current_directory).unwrap()
            .filter(|dir_res| dir_res.is_ok() )
            .filter(|dir_res| dir_res.as_ref().unwrap().path().extension().is_some())
            .filter(|dir_res| dir_res.as_ref().unwrap().path().extension().unwrap() == "iscnb")
            .map(|dir_res| dir_res.unwrap().path())
            .collect();

        if notebook_filepaths.len() == 0 {
            eprintln!("No notebook found in {}", current_directory.display());
            exit(-1);
        }

        if notebook_filepaths.len() > 1 {
            eprintln!("Multiple notebooks found in {}.\nPlease convert the other notebooks into a page or move into their own directory.", current_directory.display());
            exit(-1);
        }

        let notebook_data = match read(&notebook_filepaths[0]) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Failed to read notebook {} with error: {}", notebook_filepaths[0].display(), err);
                exit(-1);
            }
        };

        match toml::from_slice(&notebook_data) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Failed to deserialize notebook {} with error: {}", notebook_filepaths[0].display(), err);
                exit(-1);
            }
        }
    }
}
