use super::vertex::Vertex;

pub struct Arest {
    vertexes: (Vertex, Vertex),
    weight: i32,
}

impl Arest {
    pub fn from(first_vertex: Vertex, second_vertex: Vertex, weight: i32) -> Self {
        return Arest {
            vertexes: (first_vertex, second_vertex),
            weight,
        };
    }

    pub fn from_line(line: &str) -> Self {
        let formatted = line.replace(" ", "").replace("\n", ""); 
        let _formatted_as_slice = formatted.as_str();
        //todo!("Create an utils module to extract this method");
        let (_f_vtx_char, _s_vtx_char) = (_formatted_as_slice.chars().nth(0).unwrap(), _formatted_as_slice.chars().nth(1).unwrap());
        let weight = _formatted_as_slice.chars().nth(2).unwrap() as i32;
        let (mut _b1, mut _b2) = ([0; 4], [0; 4]);
        let  (f_vtx_value, s_vtx_value) = (_f_vtx_char.encode_utf8(&mut _b1),_s_vtx_char.encode_utf8(&mut _b2));

        return Arest {
            vertexes: (Vertex::new(f_vtx_value, Option::None), Vertex::new(s_vtx_value, Option::None)),
            weight
        }
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
