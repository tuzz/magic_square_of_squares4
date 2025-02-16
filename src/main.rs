#![feature(portable_simd)]

use fxhash::{FxHashSet, FxHashMap};
use std::collections::VecDeque;
use std::simd::Simd;

const LANES: usize = 16;

type SquaresVec = Vec<u64>;
type SumLookup = FxHashSet<u64>;
type SumNumbers = FxHashMap<u64, Vec<u64>>;
type SumQueue = VecDeque<u64>;

fn main() {
    let mut squares_by_residue_class = [SquaresVec::default(), SquaresVec::default(), SquaresVec::default()];
    let mut sums_by_residue_class = [SumLookup::default(), SumLookup::default(), SumLookup::default()];
    let mut sums_by_complement_class = [SumNumbers::default(), SumNumbers::default(), SumNumbers::default()];
    let mut partial_sums_to_check = SumQueue::default();

    for number in 1_u32.. {
        let square = number as u64 * number as u64;
        if square % 24 != 1 { continue; }

        //println!("{}: {}", number, square / 2 * 3);

        while let Some(partial_sum) = partial_sums_to_check.front() {
            if *partial_sum >= square + 1 { break; }

            let partial_sum = partial_sums_to_check.pop_front().unwrap();
            let center_square = partial_sum / 2;

            let center_residue = center_square % 72;
            let center_residue_class = (center_residue / 24) as usize;

            let sums = &sums_by_complement_class[center_residue_class];
            let Some(squares) = sums.get(&partial_sum) else { continue };

            // println!("{}: {:?}", center_square, squares);
        }

        let square_residue = square % 72;
        let square_residue_class = (square_residue / 24) as usize;
        let square_vector = Simd::<u64, LANES>::splat(square);

        for (i, squares) in squares_by_residue_class.iter().enumerate() {
            let sum_residue_class = (square_residue_class + i) % 3;
            let complement_class = (6 - sum_residue_class) % 3;

            let sum_lookup = &sums_by_residue_class[sum_residue_class];
            let complements = &mut sums_by_complement_class[complement_class];

            let chunks = squares.chunks_exact(LANES);
            let remainder = chunks.remainder();

            for chunk in chunks {
                let squares2_vector = Simd::from_slice(chunk);
                let partial_sum_vector = square_vector + squares2_vector;

                for i in 0..LANES {
                    let partial_sum = partial_sum_vector[i];
                    if sum_lookup.contains(&partial_sum) {
                        complements.entry(partial_sum).or_default().push(square);
                    }
                }
            }

            for &square2 in remainder {
                let partial_sum = square + square2;
                if sum_lookup.contains(&partial_sum) {
                    complements.entry(partial_sum).or_default().push(square);
                }
            }
        }

        let partial_sum = square + square;
        let sum_residue = partial_sum % 72;
        let sum_residue_class = (sum_residue / 24) as usize;

        squares_by_residue_class[square_residue_class].push(square);
        sums_by_residue_class[sum_residue_class].insert(partial_sum);
        partial_sums_to_check.push_back(partial_sum);

        if number > 600_000 { break; }
    }
}
