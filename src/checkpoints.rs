use bincode::{serialize, deserialize};
use std::collections::VecDeque;
use std::fs::{read, write, copy};
use crate::{CHECKPOINT_FREQUENCY, FILTER_BY_PRIMES};
use crate::shared_vec::*;
use crate::hashing::*;

type Squares = NoHashSet<u64>;
type SquaresByClass = [Vec<u64>; 3];
type SumsByClass = [NoHashMap<u64, SharedVec>; 3];
type CentersToCheck = VecDeque<u32>;
type NextCheckpoint = u64;
type NextNumber = u32;

const FILENAME: &str = if FILTER_BY_PRIMES { "checkpoint.filtered.bin" } else { "checkpoint.unfiltered.bin" };

pub fn read_checkpoint_or_default(log: bool) -> (Squares, SquaresByClass, SumsByClass, CentersToCheck, NextCheckpoint, NextNumber)  {
    match read(FILENAME) {
        Ok(bytes) => {
            let (mut squares, mut squares_by_class, mut sums_by_class, mut centers_to_check, checkpoint, number):
                (Squares, SquaresByClass, SumsByClass, CentersToCheck, u64, u32) = deserialize(&bytes).unwrap();

            let square = number as u64 * number as u64;

            squares.reserve(1_000_000_usize.saturating_sub(squares.len()));
            squares_by_class.iter_mut().for_each(|vec| vec.reserve(1_000_000_usize.saturating_sub(vec.len())));
            sums_by_class.iter_mut().for_each(|map| map.reserve(333_333_usize.saturating_sub(map.len())));
            centers_to_check.reserve(1_000_000_usize.saturating_sub(centers_to_check.len()));

            if log { println!("Checked all magic sums below {}. Resumed checkpoint.", square); }
            (squares, squares_by_class, sums_by_class, centers_to_check, checkpoint + CHECKPOINT_FREQUENCY, number + 1)
        },
        Err(_) => {
            let vec = Vec::with_capacity(1_000_000);
            let map = NoHashMap::with_capacity_and_hasher(333_333, BuildHasherDefault::<NoHashHasher<u64>>::default());

            let squares = NoHashSet::with_capacity_and_hasher(1_000_000, BuildHasherDefault::<NoHashHasher<u64>>::default());
            let squares_by_class = [vec.clone(), vec.clone(), vec];
            let sums_by_class = [map.clone(), map.clone(), map];
            let centers_to_check = VecDeque::with_capacity(1_000_000);

            if log { println!("No checkpoint file found. Starting the search from scratch."); }
            (squares, squares_by_class, sums_by_class, centers_to_check, CHECKPOINT_FREQUENCY, 1)
        }
    }
}

pub fn write_checkpoint(squares: Squares, squares_by_class: SquaresByClass, sums_by_class: SumsByClass, centers_to_check: CentersToCheck, next_checkpoint: u64, number: u32) -> (Squares, SquaresByClass, SumsByClass, CentersToCheck) {
    print!("Checked all magic sums below {}. ", number as u64 * number as u64);

    let _result = copy(FILENAME, format!("{}.backup", FILENAME));
    let bytes = serialize(&(&squares, &squares_by_class, &sums_by_class, &centers_to_check, next_checkpoint, number)).unwrap();

    write(FILENAME, &bytes).unwrap();
    let (squares, squares_by_class, sums_by_class, centers_to_check, _, _): (_, _, _, _, u64, u32) = read_checkpoint_or_default(false);

    println!("Wrote checkpoint.");
    (squares, squares_by_class, sums_by_class, centers_to_check)
}
