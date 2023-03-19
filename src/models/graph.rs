use super::{vertex::Vertex, arest::Arest};

pub struct Graph {
    vertexes: Vec<Vertex>,
    arests: Vec<Arest>
}

impl Graph {
    pub fn new() -> Self {
        return Graph {
            vertexes: Vec::new(),
            arests: Vec::new()
        };
    }

    pub fn arest_is_in_graph(&self, arest: &Arest) -> bool {
        return self.arests.contains(arest); 
    } 

    pub fn vertex_is_in_graph(&self, vertex: &Vertex) -> bool {
        return self.vertexes.contains(vertex);
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> () {
        if !self.vertex_is_in_graph(&vertex) {
            self.vertexes.push(vertex);
        }
        return;
    }

    pub fn add_arest(&mut self, first_vertex: Vertex, second_vertex: Vertex, weight: i32) -> () {
        let arest = Arest::new(first_vertex, second_vertex, weight);
        if !self.arest_is_in_graph(&arest) {
            self.arests.push(arest);
        }
        return;
    }

    pub fn add_fully_created_arest(&mut self, arest: Arest) {
        if self.arest_is_in_graph(&arest) {
           return; 
        }
        let _arest_clone = arest.clone();
        self.arests.push(arest);
        let vertexes = vec![_arest_clone.vertexes.0.clone(), _arest_clone.vertexes.1.clone()];
        for vtx in vertexes {
            self.add_vertex(vtx);
        }

    }
}
