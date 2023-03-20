pub struct Vertex {
    pub value: String,
    pub predecessor: Box<Option<Vertex>>,
}


impl Vertex {
    pub fn new(value: &str, predecessor: Option<Vertex>) -> Vertex {
        let _predecessor = Box::from(predecessor);
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

impl Clone for Vertex {
    fn clone(&self) -> Self {
        let _pred_ref = *self.predecessor.clone();
        let predecessor = Option::from(_pred_ref);
        return Vertex::new(self.value.as_str(), predecessor);
    }
}