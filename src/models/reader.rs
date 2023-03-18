use std::{path::PathBuf, io::{BufReader, Read}, fs::File};

use super::graph::Graph;

pub struct Reader {
    file_location: PathBuf
}

impl Reader {
    pub fn from(path: &str) -> Self {
        return Reader { file_location: PathBuf::from(path) }
    }

    pub fn create_graph(&self) -> std::io::Result<Graph> {
        let file_path = self.file_location.as_path();
        let file = File::open(file_path)?;
        let mut content = String::new();
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut content)?;
        for line in content.lines() {
            
        }

        return Ok(Graph::new());
    }
}