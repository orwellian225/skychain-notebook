//! Skychain Chapter

use std::path::PathBuf;
use std::process::exit;
use std::fs::create_dir;

use serde::{Deserialize, Serialize};

use super::page::Page;

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    identifier: String,
    #[serde(skip)]
    directory: PathBuf,

    title: String,
    pages: Vec<Page>
}

impl Chapter {
    pub fn identifier(&self) -> &String { &self.identifier }
    pub fn directory(&self) -> &PathBuf { &self.directory }
    pub fn title(&self) -> &String { &self.title }
    pub fn pages(&self) -> &Vec<Page> { &self.pages }

    pub fn identifier_mut(&mut self) -> &String { &self.identifier }
    pub fn directory_mut(&mut self) -> &PathBuf { &self.directory }
    pub fn title_mut(&mut self) -> &String { &self.title }
    pub fn pages_mut(&mut self) -> &Vec<Page> { &self.pages }
}

impl Chapter {
    pub fn create_chapter(notebook_directory: PathBuf, chapter_title: String) -> Self {
        let chapter_identifier= chapter_title
            .replace(" ", "_")
            .to_lowercase();
        let chapter_directory = notebook_directory.join(&chapter_identifier);

        let chapter = Chapter {
            identifier: chapter_identifier,
            directory: chapter_directory,
            title: chapter_title,
            pages: Vec::new()
        };

        chapter
    }

    /// To save a chapter, the following needs to happen
    /// 1. The chapter directory needs to exist
    /// 2. Each page file needs to be saved to the directory
    pub fn save_chapter(&self) {
        if  !self.directory.exists() {
            match create_dir(&self.directory) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("Failed to create the chapter directory with error: {err}");
                    exit(-1);
                }
            };
        } else {
            eprintln!("Chapter already exists");
            exit(-1);
        }

        for page in self.pages.iter() {
            page.save_page(&self.directory);
        }
    }
}

impl Default for Chapter {
    fn default() -> Self {
       Chapter { 
            identifier: "new_chapter".to_string(), 
            directory: PathBuf::default(),
            title: "New Chapter".to_string(), 
            pages: Vec::new()
        }
    }
}