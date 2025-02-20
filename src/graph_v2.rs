use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use crate::MagicSums;

pub struct Graph {
    magic_sums: MagicSums,
    ordered_squares: Vec<(Candidate, ElementIndex)>,
    additions1: Vec<(Candidate, ElementIndex, IsSquare)>,
    square_additions2: Vec<(Candidate, ElementIndex)>,
    non_square_additions3: Vec<(Candidate, ElementIndex)>,
}

type Candidate = u64;
type ElementIndex = usize;
type IsSquare = bool;
type Occurences = usize;

impl Graph {
    pub fn new() -> Self {
        Self {
            ordered_squares: vec![],
            magic_sums: MagicSums::default(),
            additions1: vec![],
            square_additions2: vec![],
            non_square_additions3: vec![],
        }
    }

    pub fn generate(&mut self, center_square: u64, bigger_numbers: Vec<u32>) {
        self.magic_sums.clear();
        self.ordered_squares.clear();
        self.additions1.clear();
        self.square_additions2.clear();
        self.non_square_additions3.clear();

        let center_sum = center_square + center_square;
        let magic_sum = center_sum + center_square;

        let bigger_squares = bigger_numbers.iter().map(|&n| n as u64 * n as u64);
        let smaller_squares = bigger_squares.clone().map(|s| center_sum - s);
        let ordered_squares = smaller_squares.rev().chain(bigger_squares);

        for square in ordered_squares {
            let (i, _) = self.magic_sums.get_or_insert_element(square, || true, |_| true).unwrap();
            self.ordered_squares.push((square, i));
        }

        'outer: for &(square1, i) in &self.ordered_squares {
            let remainder = magic_sum - square1;

            for &(square2, j) in &self.ordered_squares[i + 1..] {
                let candidate = remainder.saturating_sub(square2);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                let condition = |_| true;
                let option = self.magic_sums.get_or_insert_element(candidate, || is_square(candidate), condition);
                let (k, is_square) = option.unwrap();

                self.magic_sums.add_partnership(i, j, k);
                self.additions1.push((candidate, k, is_square));
            }
        }

        self.additions1.sort_by_key(|&(candidate, _, _)| candidate);

        'outer: for &(square, i) in &self.ordered_squares {
            let remainder = magic_sum - square;

            for &(addition, j, addition_is_square) in &self.additions1 {
                let candidate = remainder.saturating_sub(addition);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                let condition = |candidate_is_square| { candidate_is_square || addition_is_square };
                let option = self.magic_sums.get_or_insert_element(candidate, || is_square(candidate), condition);

                if let Some((k, _)) = option {
                    self.magic_sums.add_partnership(i, j, k);
                    self.square_additions2.push((candidate, k));
                }
            }
        }

        self.square_additions2.sort();

        'outer: for &(square, i) in &self.ordered_squares {
            let remainder = magic_sum - square;

            for &(addition, j) in &self.square_additions2 {
                let candidate = remainder.saturating_sub(addition);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                let condition = |candidate_is_square: bool| { !candidate_is_square };
                let option = self.magic_sums.get_or_insert_element(candidate, || is_square(candidate), condition);

                if let Some((k, _)) = option {
                    self.magic_sums.add_partnership(i, j, k);
                    self.non_square_additions3.push((candidate, k));
                }
            }
        }

        self.non_square_additions3.sort();

        'outer: for &(square, i) in &self.ordered_squares {
            let remainder = magic_sum - square;

            for &(addition, j) in &self.non_square_additions3 {
                let candidate = remainder.saturating_sub(addition);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                let condition = |candidate_is_square| candidate_is_square;
                self.magic_sums.get_or_insert_element(candidate, || is_square(candidate), condition);
            }
        }

        self.magic_sums.remove_if_less_than_two_partners();
    }
}

fn is_square(number: u64) -> bool {
    let root = number.isqrt();
    root * root == number
}
