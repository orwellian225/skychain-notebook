//! Initialise a skychain notebook 

use std::path::PathBuf;
use std::fs::create_dir;

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

        println!("Creating notebook {} in {}", &notebook.name, &notebook.directory.to_str().unwrap());
        if !notebook.directory.exists() {
            match create_dir(&notebook.directory) {
                Ok(_) => {},
                Err(err) => panic!("Failed to create the notebook directory with error {err}")
            };
        };

        notebook
    }
}