#![feature(portable_simd)]

use nohash::NoHashHasher;
use rayon::prelude::*;
use bincode::{serialize, deserialize};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::collections::{HashMap, VecDeque};
use std::hash::BuildHasherDefault;
use std::simd::Simd;
use std::sync::{Arc, Mutex};
use std::fs::{write, copy};

const SIMD_LANES: usize = 64;
const CHECKPOINTS: u64 = 10_000_000_000_000;

fn main() {
    let hasher = BuildHasherDefault::<NoHashHasher<u64>>::default();
    let map = HashMap::with_capacity_and_hasher(1_000_000, hasher);

    let mut squares_by_class = [vec![], vec![], vec![]];
    let mut sums_by_class = [map.clone(), map.clone(), map];
    let mut centers_to_check = VecDeque::new();
    let mut next_checkpoint = CHECKPOINTS;

    for number in 1_u32.. {
        let square = number as u64 * number as u64;
        if square % 24 != 1 { continue; }

        while let Some(&center) = centers_to_check.front() {
            let center_square = center as u64 * center as u64;
            let center_sum = center_square + center_square;

            // println!("{}: {} billion", number, center_square * 3 / 1_000_000_000);

            if center_sum > square { break; }
            centers_to_check.pop_front();

            let center_class = (center_square % 72 / 24) as usize;
            let complement_class = (6 - center_class) % 3;

            let sums = &mut sums_by_class[complement_class];
            let Some(numbers) = sums.remove(&hash(center_sum)) else { continue };

            // println!("{} = {}, {:?}", center_square * 3, center, numbers);
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
            let bytes = serialize(&(&squares_by_class, &sums_by_class, &centers_to_check, next_checkpoint, number)).unwrap();
            let _result = copy("checkpoint.bin", "checkpoint.backup.bin");

            write("checkpoint.bin", &bytes).unwrap();
            println!("checkpointed at magic sum {}", square);

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

#[derive(Clone, Default)]
struct SharedVec(Arc<Mutex<Vec<u32>>>);

impl Serialize for SharedVec {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.lock().unwrap().serialize(serializer)
    }
}

impl<'a> Deserialize<'a> for SharedVec {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(Arc::new(Mutex::new(Vec::deserialize(deserializer).unwrap()))))
    }
}
