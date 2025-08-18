//! skychain notebook 

use std::io::Write;
use std::path::PathBuf;
use std::fs::{create_dir, File};
use std::process::exit;

use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notebook {
    identifier: String
}

impl Notebook {
    pub fn init_notebook(current_directory: PathBuf, project_name: Option<String>) -> Notebook {
        let notebook_dir= match project_name {
            Some(val) => current_directory.join(val),
            None => current_directory.clone()
        };
        let notebook = Notebook {
            identifier: match &notebook_dir.file_name() {
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
            },
        };

        println!("Creating notebook \"{}\" in {}", &notebook.identifier, &notebook_dir.to_str().unwrap());
        if !notebook_dir.exists() {
            match create_dir(&notebook_dir) {
                Ok(_) => {},
                Err(err) => { 
                    eprintln!("Failed to create the notebook directory with error {err}");
                    exit(-1)
                }
            };
        };

        let serialized_notebook = match toml::to_string_pretty(&notebook) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Failed to convert notebook string with error {err}");
                exit(-1);
            }
        };

        let notebook_filepath = notebook_dir.join("main.iscnb");
        let mut notebook_file = match File::create(notebook_filepath) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Failed to create notebook file with error {err}");
                exit(-1);
            }
        };
        match notebook_file.write_all(serialized_notebook.as_bytes()) {
            Ok(_) => {},
            Err(err) => {
                eprintln!("Failed to write notebook data to file with error {err}");
                exit(-1);
            }
        };

        notebook
    }
}