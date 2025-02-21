#![feature(portable_simd)]

mod checkpoints;
mod hashing;
mod shared_vec;
mod patterns;

use rayon::prelude::*;
use primal::Sieve;
use checkpoints::*;
use hashing::*;
use shared_vec::*;
use patterns::*;
use std::simd::Simd;

const SIMD_LANES: usize = 64;
const CHECKPOINT_FREQUENCY: u64 = 10_000_000_000_000;
const FILTER_BY_PRIMES: bool = true;
const HIDE_KNOWN_SOLUTION: bool = true;

fn main() {
    let (mut squares, mut squares_by_class, mut sums_by_class, mut centers_to_check, mut next_checkpoint, next_number) = read_checkpoint_or_default(true);
    let sieve = Sieve::new(if FILTER_BY_PRIMES { u32::MAX as usize } else { 0 });

    for number in next_number.. {
        let square = number as u64 * number as u64;
        if square % 24 != 1 { continue; }

        if FILTER_BY_PRIMES {
            let primes = sieve.factor(number as usize).unwrap().into_iter().map(|(p, _)| p);
            if !primes.map(|p| p % 8).all(|r| r == 1 || r == 5 || r == 7) { continue; }
        }

        while let Some(&center) = centers_to_check.front() {
            let center_square = center as u64 * center as u64;
            let center_sum = center_square + center_square;

            if center_sum > square { break; } else { centers_to_check.pop_front(); }
            let center_class = (center_square % 72 / 24) as usize;

            let complement_class = (6 - center_class) % 3;
            let sums = &sums_by_class[complement_class];

            let Some(numbers) = sums.get(&hash(center_sum)) else { continue };
            let numbers = numbers.0.lock().unwrap();

            let magic_sum = center_sum + center_square;
            let center_cell = Cell::new(center_square, magic_sum);

            numbers.par_iter().enumerate().for_each(|(i, &number1)| {
                let cell1 = Cell::new(number1 as u64 * number1 as u64, magic_sum);
                let cell2 = Cell::new(center_sum - cell1.value, magic_sum);

                numbers[i + 1..].par_iter().for_each(|&number2| {
                    let cell3 = Cell::new(number2 as u64 * number2 as u64, magic_sum);
                    let cell4 = Cell::new(center_sum - cell3.value, magic_sum);

                    check_pattern_2(&squares, &center_cell, &cell1, &cell2, &cell3, &cell4);

                    check_patterns_3_4_and_6(&squares, &center_cell, &cell1, &cell2, &cell3, &cell4);
                    check_patterns_3_4_and_6(&squares, &center_cell, &cell1, &cell2, &cell4, &cell3);
                    check_patterns_3_4_and_6(&squares, &center_cell, &cell3, &cell4, &cell1, &cell2);
                    check_patterns_3_4_and_6(&squares, &center_cell, &cell3, &cell4, &cell2, &cell1);

                    check_pattern_5(&sums_by_class, &center_cell, &cell1, &cell2, &cell3, &cell4);
                    check_pattern_5(&sums_by_class, &center_cell, &cell2, &cell3, &cell4, &cell1);
                    check_pattern_5(&sums_by_class, &center_cell, &cell3, &cell4, &cell1, &cell2);
                    check_pattern_5(&sums_by_class, &center_cell, &cell4, &cell1, &cell2, &cell3);
                });
            });
        }

        let center_sum = square + square;
        let center_sum_class = (center_sum % 72 / 24) as usize;

        sums_by_class[center_sum_class].insert(hash(center_sum), SharedVec::default());
        centers_to_check.push_back(number);

        let square_class = (square % 72 / 24) as usize;
        let square_vector = Simd::splat(square);

        sums_by_class.par_iter_mut().enumerate().for_each(|(i, sums)| {
            let residue_class = (6 - square_class + i) % 3;
            let squares = &squares_by_class[residue_class];

            let chunks = squares.par_chunks_exact(SIMD_LANES);
            let remainder = chunks.remainder();

            chunks.for_each(|chunk| {
                let sum_vector = square_vector + Simd::from_slice(chunk);
                for hash in parallel_hash(sum_vector).as_array() {
                    if let Some(vec) = sums.get(hash) {
                        vec.push(number);
                    }
                }
            });

            for &square2 in remainder {
                let sum = square + square2;
                if let Some(vec) = sums.get_mut(&hash(sum)) {
                    vec.push(number);
                }
            }
        });

        squares.insert(square);
        squares_by_class[square_class].push(square);

        if square >= next_checkpoint {
            let reloaded = write_checkpoint(squares, squares_by_class, sums_by_class, centers_to_check, next_checkpoint, number);
            (squares, squares_by_class, sums_by_class, centers_to_check) = reloaded;
            next_checkpoint += CHECKPOINT_FREQUENCY;
        }
    }
}
