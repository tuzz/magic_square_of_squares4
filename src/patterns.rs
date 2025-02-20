use petgraph::graph::{DiGraph, NodeIndex};

pub struct Patterns {
    pub magic_square_of_squares: DiGraph<u64, ()>,
    pub one_cell_not_square: Vec<DiGraph<u64, ()>>,
    pub two_cells_not_square: Vec<DiGraph<u64, ()>>,
    pub one_sum_incorrect: Vec<DiGraph<u64, ()>>,
}

impl Patterns {
    pub fn new() -> Self {
        let mut graph = DiGraph::<u64, ()>::new();

        let cells = (1..=9).map(|i| graph.add_node(i)).collect::<Vec<_>>();
        let sums = (1..=8).map(|_| graph.add_node(0)).collect::<Vec<_>>();

        let equations = [
            // Rows:
            [cells[0], cells[1], cells[2], sums[0]],
            [cells[3], cells[4], cells[5], sums[1]],
            [cells[6], cells[7], cells[8], sums[2]],

            // Columns:
            [cells[0], cells[3], cells[6], sums[3]],
            [cells[1], cells[4], cells[7], sums[4]],
            [cells[2], cells[5], cells[8], sums[5]],

            // Diagonals:
            [cells[0], cells[4], cells[8], sums[6]],
            [cells[2], cells[4], cells[6], sums[7]],
        ];

        let magic_square_of_squares = Self::with_constraints(&graph, &cells, &equations, vec![], None);

        let one_cell_not_square = vec![
            Self::with_constraints(&graph, &cells, &equations, vec![4], None), // Center is not square.
            Self::with_constraints(&graph, &cells, &equations, vec![7], None), // Bottom-middle is not square.
            Self::with_constraints(&graph, &cells, &equations, vec![8], None), // Bottom-right is not square.
        ];

        let two_cells_not_square = vec![
//            Self::with_constraints(&graph, &cells, &equations, vec![4, 7], None), // Center and bottom-middle are not square.
//            Self::with_constraints(&graph, &cells, &equations, vec![4, 8], None), // Center and bottom-right are not square.
//            Self::with_constraints(&graph, &cells, &equations, vec![1, 7], None), // Top and bottom-middle are not square.
            Self::with_constraints(&graph, &cells, &equations, vec![2, 7], None), // Top-right and bottom-middle are not square.    // pattern 4
            Self::with_constraints(&graph, &cells, &equations, vec![5, 7], None), // Middle-right and bottom-middle are not square. // pattern 2
            Self::with_constraints(&graph, &cells, &equations, vec![7, 8], None), // Bottom-middle and bottom-right are not square. // pattern 3
//            Self::with_constraints(&graph, &cells, &equations, vec![6, 8], None), // Bottom-left and bottom-right are not square.
            Self::with_constraints(&graph, &cells, &equations, vec![2, 6], None), // Top-right and bottom-left are not square.      // pattern 6
        ];

        let one_sum_incorrect = vec![
            Self::with_constraints(&graph, &cells, &equations, vec![], Some(2)), // Bottom row incorrect.
            Self::with_constraints(&graph, &cells, &equations, vec![], Some(4)), // Middle column incorrect.
            Self::with_constraints(&graph, &cells, &equations, vec![], Some(8)), // Secondary diagonal incorrect.
        ];

        #[cfg(feature = "render-graphs")] {
            crate::write_svg(&magic_square_of_squares, &format!("_magic_square_of_squares.svg"));
            one_cell_not_square.iter().enumerate().for_each(|(i, g)| crate::write_svg(&g, &format!("_one_cell_not_square_{}.svg", i)));
            two_cells_not_square.iter().enumerate().for_each(|(i, g)| crate::write_svg(&g, &format!("_two_cells_not_square_{}.svg", i)));
            one_sum_incorrect.iter().enumerate().for_each(|(i, g)| crate::write_svg(&g, &format!("_one_sum_incorrect_{}.svg", i)));
        }

        Self { magic_square_of_squares, one_cell_not_square, two_cells_not_square, one_sum_incorrect }
    }

    fn with_constraints(graph: &DiGraph<u64, ()>, cells: &[NodeIndex], equations: &[[NodeIndex; 4]; 8], non_square_indexes: Vec<usize>, incorrect_sum_index: Option<usize>) -> DiGraph<u64, ()> {
        let mut graph = graph.clone();

        Self::add_cell_constraints(&mut graph, cells, non_square_indexes);
        Self::add_sum_constraints(&mut graph, equations, incorrect_sum_index);

        graph
    }

    fn add_cell_constraints(graph: &mut DiGraph<u64, ()>, cells: &[NodeIndex], non_square_indexes: Vec<usize>) {
        for (i, &cell) in cells.iter().enumerate() {
            if non_square_indexes.contains(&i) { continue; }

            graph.add_edge(cell, cell, ());
        }
    }

    fn add_sum_constraints(graph: &mut DiGraph<u64, ()>, equations: &[[NodeIndex; 4]; 8], incorrect_sum_index: Option<usize>) {
        for (i, &[x, y, z, sum]) in equations.iter().enumerate() {
            if Some(i) == incorrect_sum_index { continue; }

            graph.add_edge(x, sum, ());
            graph.add_edge(y, sum, ());
            graph.add_edge(z, sum, ());
        }
    }
}
