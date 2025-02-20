#![feature(portable_simd)]

mod checkpoints;
mod hashing;
mod shared_vec;

use rayon::prelude::*;
use primal::Sieve;
use checkpoints::*;
use hashing::*;
use shared_vec::*;
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

            numbers.par_iter().enumerate().for_each(|(i, &number1)| {
                let square1 = number1 as u64 * number1 as u64;
                let square2 = center_sum - square1;

                numbers[i + 1..].par_iter().for_each(|&number2| {
                    let square3 = number2 as u64 * number2 as u64;
                    let square4 = center_sum - square3;

                    check_pattern_2(&squares, magic_sum, center_square, square1, square2, square3, square4);

                    check_patterns_3_4_and_6(&squares, magic_sum, center_square, square1, square2, square3, square4);
                    check_patterns_3_4_and_6(&squares, magic_sum, center_square, square1, square2, square4, square3);
                    check_patterns_3_4_and_6(&squares, magic_sum, center_square, square3, square4, square1, square2);
                    check_patterns_3_4_and_6(&squares, magic_sum, center_square, square3, square4, square2, square1);

                    check_pattern_5(&sums_by_class, magic_sum, center_square, square1, square2, square3, square4);
                    check_pattern_5(&sums_by_class, magic_sum, center_square, square2, square3, square4, square1);
                    check_pattern_5(&sums_by_class, magic_sum, center_square, square3, square4, square1, square2);
                    check_pattern_5(&sums_by_class, magic_sum, center_square, square4, square1, square2, square3);
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

fn check_pattern_2(squares: &NoHashSet<u64>, magic_sum: u64, center_square: u64, top_left: u64, bottom_right: u64, top_right: u64, bottom_left: u64) {
    let magic_sum_minus_top_left = magic_sum - top_left;

    let Some(top_middle) = magic_sum_minus_top_left.checked_sub(top_right) else { return };
    let Some(middle_left) = magic_sum_minus_top_left.checked_sub(bottom_left) else { return };
    let bottom_middle = magic_sum - bottom_left - bottom_right;

    let mut num_squares = 5;
    if squares.contains(&top_middle) { num_squares += 1; }
    if squares.contains(&middle_left) { num_squares += 1; }
    if squares.contains(&bottom_middle) { num_squares += 1; }
    if num_squares < 6 { return; }

    let middle_right = magic_sum - middle_left - center_square;
    if squares.contains(&middle_right) { num_squares += 1; }
    if num_squares < 7 { return; }

    println!("{} | {} | {}", top_left, top_middle, top_right);
    println!("{} | {} | {}", middle_left, center_square, middle_right);
    println!("{} | {} | {}\n", bottom_left, bottom_middle, bottom_right);
}

fn check_pattern_5(sums_by_class: &[NoHashMap<u64, SharedVec>; 3], magic_sum: u64, center_square: u64, top_middle: u64, bottom_middle: u64, middle_left: u64, middle_right: u64) {
    let top_sum = magic_sum - top_middle;
    let left_sum = magic_sum - middle_left;
    let right_sum = magic_sum - middle_right;
    let bottom_sum = magic_sum - bottom_middle;
    let center_sum = magic_sum - center_square;

    for sums in sums_by_class {
        if let Some(numbers) = sums.get(&hash(top_sum)) {
            let numbers = numbers.0.lock().unwrap();

            for &number in numbers.iter() { // TODO: SIMD
                let top_left = number as u64 * number as u64;
                let top_right = top_sum - top_left;

                let Some(bottom_left) = left_sum.checked_sub(top_left) else { continue };
                let Some(bottom_right) = right_sum.checked_sub(top_right) else { continue };
                if bottom_left + bottom_right != bottom_sum { continue };
                if top_left + bottom_right != center_sum { continue; }
                if top_right + bottom_left != center_sum { continue; }

                println!("{} | {} | {}", top_left, top_middle, top_right);
                println!("{} | {} | {}", middle_left, center_square, middle_right);
                println!("{} | {} | {}\n", bottom_left, bottom_middle, bottom_right);
            }
        }
    }
}

fn check_patterns_3_4_and_6(squares: &NoHashSet<u64>, magic_sum: u64, center_square: u64, top_right: u64, bottom_left: u64, middle_left: u64, middle_right: u64) {
    let magic_sum_minus_bottom_left = magic_sum - bottom_left;
    let magic_sum_minus_top_right = magic_sum - top_right;

    let Some(top_left) = magic_sum_minus_bottom_left.checked_sub(middle_left) else { return };
    let Some(bottom_right) = magic_sum_minus_top_right.checked_sub(middle_right) else { return };
    let Some(top_middle) = magic_sum_minus_top_right.checked_sub(top_left) else { return };

    let mut num_squares = 5;
    if squares.contains(&top_left) { num_squares += 1; }
    if squares.contains(&bottom_right) { num_squares += 1; }
    if squares.contains(&top_middle) { num_squares += 1; }
    if num_squares < 6 { return; }

    let bottom_middle = magic_sum_minus_bottom_left - bottom_right;
    if squares.contains(&bottom_middle) { num_squares += 1; }
    if num_squares < 7 { return; }

    if HIDE_KNOWN_SOLUTION {
        let k2 = magic_sum / 541_875;
        let k = k2.isqrt();
        if k * k == k2 { return; }
    }

    println!("{} | {} | {}", top_left, top_middle, top_right);
    println!("{} | {} | {}", middle_left, center_square, middle_right);
    println!("{} | {} | {}\n", bottom_left, bottom_middle, bottom_right);
}
