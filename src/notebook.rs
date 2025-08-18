//! skychain notebook 

use std::io::Write;
use std::path::PathBuf;
use std::fs::{create_dir, File};

use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notebook {
    name: String,
    directory: PathBuf
}

impl Notebook {
    pub fn init_notebook(current_directory: PathBuf, project_name: Option<String>) -> Notebook {
        let notebook_dir= match project_name {
            Some(val) => current_directory.join(val),
            None => current_directory.clone()
        };
        let notebook = Notebook {
            name: match &notebook_dir.file_name() {
                Some(val) => match val.to_str() {
                    Some(val) => val.to_string(),
                    None => panic!("Project directory {} is not valid unicode", &notebook_dir.as_os_str().to_str().unwrap())
                },
                None => panic!("Project directory {} cannot be used to identify a project name", &notebook_dir.as_os_str().to_str().unwrap())
            },
            directory: notebook_dir
        };

        println!("Creating notebook \"{}\" in {}", &notebook.name, &notebook.directory.to_str().unwrap());
        if !notebook.directory.exists() {
            match create_dir(&notebook.directory) {
                Ok(_) => {},
                Err(err) => panic!("Failed to create the notebook directory with error {err}")
            };
        };

        let serialized_notebook = match toml::to_string_pretty(&notebook) {
            Ok(val) => val,
            Err(err) => {
                panic!("Failed to convert notebook string with error {err}");
            }
        };

        let notebook_filepath = notebook.directory.join("main.iscnb");
        let mut notebook_file = match File::create(notebook_filepath) {
            Ok(val) => val,
            Err(err) => panic!("Failed to create notebook file with error {err}")
        };
        match notebook_file.write_all(serialized_notebook.as_bytes()) {
            Ok(_) => {},
            Err(err) => panic!("Failed to write notebook data to file with error {err}")
        };

        notebook
    }
}