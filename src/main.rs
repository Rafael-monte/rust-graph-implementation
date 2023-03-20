use models::{reader::Reader};

mod models;
mod utils;


fn main() {
    let reader = Reader::from("/home/rafael/Documentos/projetos/rust/rust-graph-implementation/src/resources/graph.txt");
    let graph = reader.create_graph();
    match graph {
        Ok(graph) => {
            graph.get_arests().iter().for_each(|arest|{
                let (f_vtx, s_vtx) = &arest.vertexes;
                println!("{}", format!("{}->{} (weight: {})", f_vtx.value, s_vtx.value, arest.weight))
            });
        },
        Err(_e) => {}
    }
}
