#![feature(portable_simd)]

mod shared_vec;

use nohash::NoHashHasher;
use rayon::prelude::*;
use bincode::{serialize, deserialize};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use shared_vec::*;
use std::collections::{HashMap, VecDeque};
use std::hash::BuildHasherDefault;
use std::simd::Simd;
use std::sync::{Arc, Mutex};
use std::fs::{read, write, copy};

const SIMD_LANES: usize = 64;
const CHECKPOINTS: u64 = 10_000_000_000_000;

fn main() {
    let (mut squares_by_class, mut sums_by_class, mut centers_to_check, mut next_checkpoint, next_number) = match read("checkpoint.bin") {
        Ok(bytes) => {
            let (squares, sums, centers, checkpoint, number): (_, _, _, u64, u32) = deserialize(&bytes).unwrap();
            let square = number as u64 * number as u64;
            println!("Checked all magic sums below {}. Resumed checkpoint.", square);

            (squares, sums, centers, checkpoint + CHECKPOINTS, number + 1)
        },
        Err(_) => {
            println!("No checkpoint file found. Starting the search from scratch.");
            let hasher = BuildHasherDefault::<NoHashHasher<u64>>::default();
            let map = HashMap::with_capacity_and_hasher(1_000_000, hasher);
            let squares_by_class = [vec![], vec![], vec![]];
            let sums_by_class = [map.clone(), map.clone(), map];
            let centers_to_check = VecDeque::new();
            let next_checkpoint = CHECKPOINTS;
            let next_number = 1;

            (squares_by_class, sums_by_class, centers_to_check, next_checkpoint, next_number)
        }
    };

    for number in next_number.. {
        let square = number as u64 * number as u64;
        if square % 24 != 1 { continue; }

        while let Some(&center) = centers_to_check.front() {
            let center_square = center as u64 * center as u64;
            let center_sum = center_square + center_square;

            if center_sum > square { break; }
            centers_to_check.pop_front();

            let center_class = (center_square % 72 / 24) as usize;
            let complement_class = (6 - center_class) % 3;

            let sums = &mut sums_by_class[complement_class];
            let Some(SharedVec(numbers)) = sums.remove(&hash(center_sum)) else { continue };

            let numbers = Mutex::into_inner(Arc::into_inner(numbers).unwrap()).unwrap();
            // println!("{}, {:?}", center, numbers);
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
                        vec.0.lock().unwrap().push(number);
                    }
                }
            });

            for &square2 in remainder {
                let sum = square + square2;
                if let Some(vec) = sums.get_mut(&hash(sum)) {
                    vec.0.lock().unwrap().push(number);
                }
            }
        });

        squares_by_class[square_class].push(square);

        if square >= next_checkpoint {
            print!("Checked all magic sums below {}. ", square);

            let _result = copy("checkpoint.bin", "checkpoint.backup.bin");
            let bytes = serialize(&(&squares_by_class, &sums_by_class, &centers_to_check, next_checkpoint, number)).unwrap();

            write("checkpoint.bin", &bytes).unwrap();
            println!("Wrote checkpoint.");

            next_checkpoint += CHECKPOINTS;
        }
    }
}

const PRIME: u64 = 11400714785074694791;
const PRIME_VECTOR: Simd::<u64, SIMD_LANES> = Simd::splat(PRIME);

fn parallel_hash(vector: Simd::<u64, SIMD_LANES>) -> Simd::<u64, SIMD_LANES> {
    let round1 = vector ^ (vector >> 33);
    let round2 = round1 * PRIME_VECTOR;
    round2 ^ (round2 >> 33)
}

fn hash(value: u64) -> u64 {
    let round1 = value ^ value.wrapping_shr(33);
    let round2 = round1.wrapping_mul(PRIME);
    round2 ^ round2.wrapping_shr(33)
}
