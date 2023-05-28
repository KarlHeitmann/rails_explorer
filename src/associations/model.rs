use std::path::PathBuf;
use std::io::prelude::*;

use ratatui::{
    text::{Span, Spans},
    widgets::{
        ListItem,
    },
};

#[derive(Debug)]
pub struct Model {
    path_buf: PathBuf,
    name: String,
}

impl From<&Model> for ListItem<'_> {
    fn from(model: &Model) -> Self {
        ListItem::new(Spans::from(
            vec![
                Span::from(format!("{}", model.name)),
            ]
        ))
    }
}


impl Model {
    pub fn new(path_buf: PathBuf) -> Self {
        let s: &str = path_buf.to_str().unwrap();
        let (_, name) = s.rsplit_once("/").unwrap();

        Self {
            path_buf: path_buf.clone(),
            name: name.to_string(),
        }
    }
    pub fn parse_and_display(&self)  -> Result<String, Box<dyn std::error::Error>> {
        let path = std::path::Path::new(&self.path_buf);
        // let mut file = std::fs::File::open(&self.path_buf)?;
        // let mut file = std::fs::File::open(&path)?;
        let mut file = std::fs::File::open(&path)?;
        let mut contents = String::new();
        let _num_bytes_read = file.read_to_string(&mut contents)?;
        let lines = contents.split("\n");

        let mut has_many: Vec<String> = vec![];
        let mut belongs_to: Vec<String> = vec![];

        for line in lines {
            if line.contains("has_many") { has_many.push(line.to_string()); }
            if line.contains("belongs_to") { belongs_to.push(line.to_string()); }
        }

        let output = format!("\nhas_many vec: {:?}\nbelongs_to vec: {:?}", has_many, belongs_to);
        log::debug!("{}", output);

        Ok(format!("name: {}\npath: {}", self.name, self.path_buf.display()))
        // Ok(output)
    }
    pub fn display_string(&self) -> String {
        self.name.clone()
    }
}

impl From<&Model> for String {
    fn from(model: &Model) -> Self {
        format!("{}", model.name)
    }
}


