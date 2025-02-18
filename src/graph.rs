use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

pub fn generate_graph(center_square: u64, bigger_numbers: Vec<u32>) -> DiGraph<u64, ()> {
    let center_sum = center_square + center_square;
    let magic_sum = center_sum + center_square;

    let bigger_squares = bigger_numbers.iter().map(|&n| n as u64 * n as u64).collect::<Vec<_>>();
    let smaller_squares = bigger_squares.iter().map(|&s| center_sum - s).collect::<Vec<_>>();

    let ordered_squares = smaller_squares.iter().rev().chain(&bigger_squares).copied().collect::<Vec<_>>();
    let num_squares = ordered_squares.len();

    let mut graph = DiGraph::<u64, ()>::new();
    let mut nodes: HashMap<u64, NodeIndex> = HashMap::with_capacity(num_squares + num_squares * num_squares);

    let ordered_nodes = ordered_squares.iter().map(|&square| {
        let node = graph.add_node(square);

        nodes.insert(square, node);
        graph.add_edge(node, node, ());

        node
    }).collect::<Vec<_>>();

    for (i, (&square1, &square1_node)) in ordered_squares.iter().zip(&ordered_nodes).enumerate() {
        for (&square2, &square2_node) in ordered_squares[i + 1..].iter().zip(&ordered_nodes[i + 1..]) {
            let remainder = (magic_sum - square1).saturating_sub(square2);
            if remainder == 0 { break; }

            let remainder_node = *nodes.entry(remainder).or_insert_with(|| {
                let node = graph.add_node(remainder);
                let sqrt = remainder.isqrt();

                let is_square = sqrt * sqrt == remainder;
                if is_square { graph.add_edge(node, node, ()); }

                node
            });

            let magic_sum_node = graph.add_node(0);

            graph.add_edge(square1_node, magic_sum_node, ());
            graph.add_edge(square2_node, magic_sum_node, ());
            graph.add_edge(remainder_node, magic_sum_node, ());
            graph.add_edge(square2_node, square1_node, ());

            match square1 > remainder {
                true => graph.add_edge(square1_node, remainder_node, ()),
                false => graph.add_edge(remainder_node, square1_node, ()),
            };

            match square2 > remainder {
                true => graph.add_edge(square2_node, remainder_node, ()),
                false => graph.add_edge(remainder_node, square2_node, ()),
            };
        }
    }

    graph
}

#[cfg(feature = "render-graphs")]
pub fn write_svg(graph: &DiGraph<u64, ()>, center_square: u64) {
    let dot_config = petgraph::dot::Config::EdgeNoLabel;
    let svg_format = graphviz_rust::cmd::Format::Svg.into();

    let dot_graph = format!("{:?}", petgraph::dot::Dot::with_config(&graph, &[dot_config]));
    let svg_graph = graphviz_rust::exec_dot(dot_graph, vec![svg_format]).unwrap();

    let kind = if crate::FILTER_BY_PRIMES { "filtered" } else { "unfiltered" };
    let filename = format!("graphs/magic_sum_{}.{}.svg", center_square * 3, kind);

    const CREATED_DIR: std::cell::OnceCell<()> = std::cell::OnceCell::new();
    CREATED_DIR.get_or_init(|| { let _ = std::fs::create_dir_all("graphs"); });

    std::fs::write(filename, svg_graph).unwrap();
}
