use petgraph::graph::DiGraph;
use vf2::Graph;
use vf2::{Vf2Builder, subgraph_isomorphisms};
use crate::patterns::Patterns;

pub fn match_subgraphs(center_square: u64, graph: &DiGraph<u64, ()>, patterns: &Patterns) {
    let magic_sum = center_square * 3;
    if magic_sum != 541875 { return; }

    for (i, subgraph) in patterns.two_cells_not_square.iter().enumerate() {
        if subgraph.node_count() > graph.node_count() { continue; }

        println!("matching for {}", magic_sum);
        let isomorphisms = subgraph_isomorphisms(subgraph, graph).node_eq(|&a, &b| (a == 0) == (b == 0));

        if let Some(isomorphism) = isomorphisms.iter().next() {
            println!("{}: {:?}", magic_sum, isomorphism);
        }
        println!("done");
    }
}
