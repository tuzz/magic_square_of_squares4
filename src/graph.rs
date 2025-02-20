use petgraph::graph::{DiGraph, NodeIndex};
use crate::MagicSums;
use crate::hashing::*;
use std::collections::HashMap;

pub fn generate_graph(center_square: u64, magic_sums: &MagicSums) -> DiGraph<u64, ()> {
    let mut graph = DiGraph::<u64, ()>::new();
    let magic_sum = center_square * 3;

    let nodes = magic_sums.elements.iter().map(|element| {
        let node = graph.add_node(element.number);

        if element.is_square {
            graph.add_edge(node, node, ());
        }

        node
    }).collect::<Vec<_>>();

    for (i, element) in magic_sums.elements.iter().enumerate() {
        let this_node = nodes[i];

        for &p in &element.partners {
            let partner = &magic_sums.elements[p];
            if partner.number > element.number { continue; }

            let remainder = magic_sum - element.number - partner.number;
            if remainder > element.number { continue; }

            let partner1_node = nodes[p];
            let partner2_node = nodes[*magic_sums.lookup.get(&hash(remainder)).unwrap()];
            let sum_node = graph.add_node(0);

            graph.add_edge(this_node, sum_node, ());
            graph.add_edge(partner1_node, sum_node, ());
            graph.add_edge(partner2_node, sum_node, ());
        }
    }

    #[cfg(feature = "render-graphs")] {
        let kind = if crate::FILTER_BY_PRIMES { "filtered" } else { "unfiltered" };
        write_svg(&graph, &format!("magic_sum_{}.{}.svg", magic_sum, kind))
    }

    graph
}

#[cfg(feature = "render-graphs")]
pub fn write_svg(graph: &DiGraph<u64, ()>, filename: &str) {
    let dot_config = petgraph::dot::Config::EdgeNoLabel;
    let svg_format = graphviz_rust::cmd::Format::Svg.into();

    let dot_graph = format!("{:?}", petgraph::dot::Dot::with_config(&graph, &[dot_config]));
    let svg_graph = graphviz_rust::exec_dot(dot_graph, vec![svg_format]).unwrap();

    const CREATED_DIR: std::cell::OnceCell<()> = std::cell::OnceCell::new();
    CREATED_DIR.get_or_init(|| { let _ = std::fs::create_dir_all("graphs"); });

    std::fs::write(format!("graphs/{}", filename), svg_graph).unwrap();
}
