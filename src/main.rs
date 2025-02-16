#![feature(portable_simd)]

use nohash::NoHashHasher;
use std::collections::{HashMap, VecDeque};
use std::hash::BuildHasherDefault;
use std::simd::Simd;

const LANES: usize = 64;

fn main() {
    let hasher = BuildHasherDefault::<NoHashHasher<u64>>::default();
    let map = HashMap::with_capacity_and_hasher(1_000_000, hasher);

    let mut squares_by_class = [vec![], vec![], vec![]];
    let mut sums_by_class = [map.clone(), map.clone(), map];
    let mut sums_to_check = VecDeque::new();

    for number in 1_u32.. {
        let square = number as u64 * number as u64;
        if square % 24 != 1 { continue; }

        while let Some(sum) = sums_to_check.front() {
            if *sum >= square + 1 { break; }
            let sum = sums_to_check.pop_front().unwrap();

            let center_square = sum / 2;
            let center_class = (center_square % 72 / 24) as usize;

            let complement_class = (6 - center_class) % 3;
            let sums = &sums_by_class[complement_class];

            let Some(squares) = sums.get(&hash(sum)) else { continue };

            //println!("{} = {} + ... {:?}", center_square * 3, center_square, squares);
        }

        let center_sum = square + square;
        let center_sum_class = (center_sum % 72 / 24) as usize;

        sums_by_class[center_sum_class].insert(hash(center_sum), vec![]);
        sums_to_check.push_back(center_sum);

        let square_class = (square % 72 / 24) as usize;
        let square_vector = Simd::splat(square);

        for (i, squares) in squares_by_class.iter().enumerate() {
            let sum_class = (square_class + i) % 3;
            let sums = &mut sums_by_class[sum_class];

            let chunks = squares.chunks_exact(LANES);
            let remainder = chunks.remainder();

            for chunk in chunks {
                let sum_vector = square_vector + Simd::from_slice(chunk);
                let hash_keys = parallel_hash(sum_vector);

                for i in 0..LANES {
                    if let Some(vec) = sums.get_mut(&hash_keys[i]) {
                        vec.push(square);
                    }
                }
            }

            for &square2 in remainder {
                let sum = square + square2;
                let hash_key = hash(sum);

                if let Some(vec) = sums.get_mut(&hash_key) {
                    vec.push(square);
                }
            }
        }

        squares_by_class[square_class].push(square);

        if number > 600_000 { break; }
    }
}

const PRIME: u64 = 11400714785074694791;
const PRIME_VECTOR: Simd::<u64, LANES> = Simd::splat(PRIME);

fn parallel_hash(vector: Simd::<u64, LANES>) -> Simd::<u64, LANES> {
    let round1 = vector ^ (vector >> 33);
    let round2 = round1 * PRIME_VECTOR;
    let round3 = round2 ^ (round2 >> 33);
    round3
}

fn hash(value: u64) -> u64 {
    let round1 = value ^ value.wrapping_shr(33);
    let round2 = round1.wrapping_mul(PRIME);
    let round3 = round2 ^ round2.wrapping_shr(33);
    round3
}
