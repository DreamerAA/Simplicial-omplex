use crate::simplicial_complex::icomplex::IComplex;
pub mod simplicial_complex;

use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use std::string::String;
use std::collections::HashMap;
use uuid::Uuid;


pub struct Complex{
    pub cells: Vec<Uuid>,
    pub graph: Graph<(), (), petgraph::Undirected>,
}

impl Complex{
    fn new(graph: Graph<(), (), petgraph::Undirected>) -> Complex {
        let mut complex: Complex = Complex{cells: Vec::new(), graph: Graph::new_undirected()};
        
        // main_node_id_to_node_ind
        let mut mn_id_ni: HashMap<Uuid,NodeIndex> = HashMap::new();
        // complex_node_id_to_node_ind
        let mut cn_id_ni: HashMap<Uuid,NodeIndex> = HashMap::new();
        for start in graph.node_indices() {
            let uid = Uuid::new_v4();
            let cni = complex.graph.add_node(());
            
            mn_id_ni.insert(uid, start);
            cn_id_ni.insert(uid, cni);
        }
        complex
    }
}

impl IComplex for Complex{
    fn cells(&self) -> &[Uuid]{
        &self.cells
    }
    fn dimension(&self, id:Uuid) -> u32{
        1
    }
    fn facets(&self, id:Uuid) -> &[Uuid]{
        &self.cells
    }
    fn cofacets(&self, id:Uuid) -> &[Uuid]{
        &self.cells
    }
    fn delete_cells(&self, ids: Vec<Uuid>){

    }
    fn add_cell(&self, facets: Vec<Uuid>){

    }
}

// #[cfg(test)]
// mod test_complex {
//     #[test]
//     fn it_works() {
        
//     }
// }

fn main() {

    let mut graph : Graph<(), (), petgraph::Undirected> = Graph::new_undirected();
    graph.extend_with_edges(&[
        (1,2),(2,3),(3,1)
    ]);
    let complex = Complex::new(graph);
}
