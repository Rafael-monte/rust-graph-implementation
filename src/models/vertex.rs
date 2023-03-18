pub struct Vertex {
    pub value: String,
    pub predecessor: Box<Vertex>,
}


impl Vertex {
    pub fn new(value: &str, predecessor: Option<Vertex>) -> Vertex {
        let _predecessor = Box::from(predecessor.unwrap());
        return Vertex {
            value: String::from(value),
            predecessor: _predecessor
        };
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        return self.value == other.value;
    }
}