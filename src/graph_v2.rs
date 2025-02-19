use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub struct Graph {
    ordered_squares: Vec<Candidate>,
    map: HashMap<Candidate, (IsSquare, Occurences)>,
    additions1: Vec<(Candidate, IsSquare)>,
    square_additions2: Vec<Candidate>,
    non_square_additions3: Vec<Candidate>,
}

type Candidate = u64;
type IsSquare = bool;
type Occurences = usize;

impl Graph {
    pub fn new() -> Self {
        Self {
            ordered_squares: vec![],
            map: HashMap::new(),
            additions1: vec![],
            square_additions2: vec![],
            non_square_additions3: vec![],
        }
    }

    pub fn generate(&mut self, center_square: u64, bigger_numbers: Vec<u32>) {
        self.ordered_squares.clear();
        self.map.clear();
        self.additions1.clear();
        self.square_additions2.clear();
        self.non_square_additions3.clear();

        let center_sum = center_square + center_square;
        let magic_sum = center_sum + center_square;

        let bigger_squares = bigger_numbers.iter().map(|&n| n as u64 * n as u64);
        let smaller_squares = bigger_squares.clone().map(|s| center_sum - s);

        self.ordered_squares.extend(smaller_squares.rev().chain(bigger_squares));
        self.map.extend(self.ordered_squares.iter().map(|&s| (s, (true, 1))));

        'outer: for (i, &square1) in self.ordered_squares.iter().enumerate() {
            let remainder = magic_sum - square1;

            for (j, &square2) in self.ordered_squares[i + 1..].iter().enumerate() {
                let candidate = remainder.saturating_sub(square2);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                self.map.entry(candidate)
                    .and_modify(|(_is_square, occurences)| *occurences += 1)
                    .or_insert_with(|| {
                        let is_square = is_perfect_square(candidate);
                        self.additions1.push((candidate, is_square));
                        (is_square, 1)
                    });
            }
        }

        self.additions1.sort();

        'outer: for (i, &square) in self.ordered_squares.iter().enumerate() {
            let remainder = magic_sum - square;

            for (j, &(addition, addition_is_square)) in self.additions1.iter().enumerate() {
                let candidate = remainder.saturating_sub(addition);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                match self.map.entry(candidate) {
                    Occupied(mut entry) => entry.get_mut().1 += 1,
                    Vacant(entry) => {
                        let candidate_is_square = is_perfect_square(candidate);
                        if candidate_is_square || addition_is_square {
                            entry.insert((candidate_is_square, 1));
                            if candidate_is_square { self.square_additions2.push(candidate); }
                        }
                    },
                }
            }
        }

        self.square_additions2.sort();

        'outer: for (i, &square) in self.ordered_squares.iter().enumerate() {
            let remainder = magic_sum - square;

            for (j, &addition) in self.square_additions2.iter().enumerate() {
                let candidate = remainder.saturating_sub(addition);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                self.map.entry(candidate)
                    .and_modify(|(_is_square, occurences)| *occurences += 1)
                    .or_insert_with(|| {
                        let is_square = is_perfect_square(candidate);
                        if !is_square { self.non_square_additions3.push(candidate); }
                        (is_square, 1)
                    });
            }
        }

        self.non_square_additions3.sort();

        'outer: for (i, &square) in self.ordered_squares.iter().enumerate() {
            let remainder = magic_sum - square;

            for (j, &addition) in self.non_square_additions3.iter().enumerate() {
                let candidate = remainder.saturating_sub(addition);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                match self.map.entry(candidate) {
                    Occupied(mut entry) => entry.get_mut().1 += 1,
                    Vacant(entry) => {
                        let is_square = is_perfect_square(candidate);
                        if is_square { entry.insert((is_square, 1)); }
                    },
                }
            }
        }
    }
}

fn is_perfect_square(number: u64) -> bool {
    let root = number.isqrt();
    root * root == number
}
