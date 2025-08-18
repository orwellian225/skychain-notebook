//! skychain page
//! 
use std::io::Write;
use std::path::PathBuf;
use std::fs::File;
use std::process::exit;

use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {}

impl Page {
    pub fn create_page(directory: PathBuf, page_name: String) -> Page {
        let page = Page {};

        println!("Creating new page");
        let serialized_page = match toml::to_string_pretty(&page) {
            Ok(val) => val,
            Err(err) => {
                eprintln!("Failed to convert page string with error {err}");
                exit(-1);
            }
        };

        let page_filepath = directory.join(format!("{page_name}.iscpg"));
        let mut page_file = match File::create(page_filepath) {
            Ok(val) => val,
            Err(err) => { 
                eprintln!("Failed to create page file with error {err}");
                exit(-1);
            }
        };
        match page_file.write_all(serialized_page.as_bytes()) {
            Ok(_) =>  {},
            Err(err) => { 
                eprintln!("failed to write page data to file with error {err}");
                exit(-1);
            }
        }
        
        page
    }
}