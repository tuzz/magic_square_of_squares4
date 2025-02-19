use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub struct Graph {

}

impl Graph {
    pub fn new() -> Self {
        Self { }
    }

    pub fn generate(&mut self, center_square: u64, bigger_numbers: Vec<u32>) {
        let center_sum = center_square + center_square;
        let magic_sum = center_sum + center_square;

        let bigger_squares = bigger_numbers.iter().map(|&n| n as u64 * n as u64);
        let smaller_squares = bigger_squares.clone().map(|s| center_sum - s);

        let ordered_squares = smaller_squares.rev().chain(bigger_squares).collect::<Vec<_>>();
        let mut map = ordered_squares.iter().map(|&s| (s, (true, 1))).collect::<HashMap<_, _>>();
        let mut additions1 = vec![];

        'outer: for (i, &square1) in ordered_squares.iter().enumerate() {
            let remainder = magic_sum - square1;

            for (j, &square2) in ordered_squares[i + 1..].iter().enumerate() {
                let candidate = remainder.saturating_sub(square2);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                map.entry(candidate)
                    .and_modify(|(_is_square, occurences)| *occurences += 1)
                    .or_insert_with(|| {
                        let is_square = is_perfect_square(candidate);
                        additions1.push((candidate, is_square));
                        (is_square, 1)
                    });
            }
        }

        additions1.sort();
        let mut square_additions2 = vec![];

        'outer: for (i, &square) in ordered_squares.iter().enumerate() {
            let remainder = magic_sum - square;

            for (j, &(addition, addition_is_square)) in additions1.iter().enumerate() {
                let candidate = remainder.saturating_sub(addition);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                match map.entry(candidate) {
                    Occupied(mut entry) => entry.get_mut().1 += 1,
                    Vacant(entry) => {
                        let candidate_is_square = is_perfect_square(candidate);
                        if candidate_is_square || addition_is_square {
                            entry.insert((candidate_is_square, 1));
                            if candidate_is_square { square_additions2.push(candidate); }
                        }
                    },
                }
            }
        }

        square_additions2.sort();
        let mut non_square_additions3 = vec![];

        'outer: for (i, &square) in ordered_squares.iter().enumerate() {
            let remainder = magic_sum - square;

            for (j, &addition) in square_additions2.iter().enumerate() {
                let candidate = remainder.saturating_sub(addition);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                map.entry(candidate)
                    .and_modify(|(_is_square, occurences)| *occurences += 1)
                    .or_insert_with(|| {
                        let is_square = is_perfect_square(candidate);
                        if !is_square { non_square_additions3.push(candidate); }
                        (is_square, 1)
                    });
            }
        }

        non_square_additions3.sort();

        'outer: for (i, &square) in ordered_squares.iter().enumerate() {
            let remainder = magic_sum - square;

            for (j, &addition) in non_square_additions3.iter().enumerate() {
                let candidate = remainder.saturating_sub(addition);
                if candidate == 0 { if j == 0 { break 'outer; } else { break; } }

                match map.entry(candidate) {
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
