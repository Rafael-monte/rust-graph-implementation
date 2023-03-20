use crate::utils;

use super::vertex::Vertex;

pub struct Arest {
    pub vertexes: (Vertex, Vertex),
    pub weight: i32,
}

impl Arest {
    pub fn new(first_vertex: Vertex, second_vertex: Vertex, weight: i32) -> Self {
        return Arest {
            vertexes: (first_vertex, second_vertex),
            weight,
        };
    }
    pub fn from_line(line: &str) -> Self {
        let formatted = line.replace(" ", "").replace("\n", ""); 
        let f_vtx_value = utils::char_to_string(&formatted.chars().nth(0).unwrap());
        let s_vtx_value = utils::char_to_string(&formatted.chars().nth(1).unwrap());
        let weight = utils::char_to_i32(&formatted.chars().nth(2).unwrap()).unwrap();
        return Self::new(
            Vertex::new(f_vtx_value.as_str(), Option::None), 
            Vertex::new(s_vtx_value.as_str(), Option::None), 
            weight
        );
    }
}

impl PartialEq for Arest {
    fn eq(&self, other: &Arest) -> bool {
        let (not_equal, equal): (bool, bool) = (false, true);
        if self.vertexes.0 != other.vertexes.0 {
            return not_equal;
        }
        if self.vertexes.1 != other.vertexes.1 {
            return not_equal;
        }
        if self.weight != other.weight {
            return not_equal;
        }
        return equal;
    }
}

impl Clone for Arest {
    fn clone(&self) -> Self {
        return Self::new(self.vertexes.0.clone(), self.vertexes.1.clone(), self.weight);
    }
}