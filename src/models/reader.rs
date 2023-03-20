use std::{path::PathBuf, io::{BufReader, Read}, fs::File};

use super::{graph::Graph, arest::Arest};

pub struct Reader {
    file_location: PathBuf
}

impl Reader {
    pub fn from(path: &str) -> Self {
        return Reader { file_location: PathBuf::from(path) }
    }

    pub fn create_graph(&self) -> std::io::Result<Graph> {
        let mut graph: Graph = Graph::new();
        let file_path = self.file_location.as_path();
        let _opening_file = File::open(file_path);
        if _opening_file.is_err() {
            eprintln!("Ocorreu um erro ao abrir arquivo de grafos");
            return Err(_opening_file.unwrap_err());
    
        } 
        let file: Option<File> = Option::from(_opening_file.unwrap());
        let mut content = String::new();
        let mut reader = BufReader::new(file.unwrap());
        reader.read_to_string(&mut content)?;
        for line in content.lines() {
            let arest: Arest = Arest::from_line(line);
            graph.add_fully_created_arest(arest);
        }
        return Ok(graph);
    }
}