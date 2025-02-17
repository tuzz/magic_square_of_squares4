use bincode::{serialize, deserialize};
use nohash::NoHashHasher;
use std::collections::{HashMap, VecDeque};
use std::hash::BuildHasherDefault;
use std::fs::{read, write, copy};
use crate::CHECKPOINT_FREQUENCY;
use crate::shared_vec::*;

type SquaresByClass = [Vec<u64>; 3];
type SumsByClass = [HashMap<u64, SharedVec, BuildHasherDefault::<NoHashHasher<u64>>>; 3];
type CentersToCheck = VecDeque<u32>;
type NextCheckpoint = u64;
type NextNumber = u32;

pub fn read_checkpoint_or_default(log: bool) -> (SquaresByClass, SumsByClass, CentersToCheck, NextCheckpoint, NextNumber)  {
    match read("checkpoint.bin") {
        Ok(bytes) => {
            let (mut squares, mut sums, mut centers, checkpoint, number): (SquaresByClass, SumsByClass, CentersToCheck, u64, u32) = deserialize(&bytes).unwrap();
            let square = number as u64 * number as u64;

            squares.iter_mut().for_each(|vec| vec.reserve(1_000_000_usize.saturating_sub(vec.len())));
            sums.iter_mut().for_each(|map| map.reserve(333_333_usize.saturating_sub(map.len())));
            centers.reserve(1_000_000_usize.saturating_sub(centers.len()));

            if log { println!("Checked all magic sums below {}. Resumed checkpoint.", square); }
            (squares, sums, centers, checkpoint + CHECKPOINT_FREQUENCY, number + 1)
        },
        Err(_) => {
            let vec = Vec::with_capacity(1_000_000);
            let map = HashMap::with_capacity_and_hasher(333_333, BuildHasherDefault::<NoHashHasher<u64>>::default());

            let squares_by_class = [vec.clone(), vec.clone(), vec];
            let sums_by_class = [map.clone(), map.clone(), map];
            let centers_to_check = VecDeque::with_capacity(1_000_000);

            if log { println!("No checkpoint file found. Starting the search from scratch."); }
            (squares_by_class, sums_by_class, centers_to_check, CHECKPOINT_FREQUENCY, 1)
        }
    }
}

pub fn write_checkpoint(squares_by_class: SquaresByClass, sums_by_class: SumsByClass, centers_to_check: CentersToCheck, next_checkpoint: u64, number: u32) -> (SquaresByClass, SumsByClass, CentersToCheck) {
    print!("Checked all magic sums below {}. ", number as u64 * number as u64);

    let _result = copy("checkpoint.bin", "checkpoint.backup.bin");
    let bytes = serialize(&(&squares_by_class, &sums_by_class, &centers_to_check, next_checkpoint, number)).unwrap();

    write("checkpoint.bin", &bytes).unwrap();
    let (squares_by_class, sums_by_class, centers_to_check, _, _): (_, _, _, u64, u32) = read_checkpoint_or_default(false);

    println!("Wrote checkpoint.");
    (squares_by_class, sums_by_class, centers_to_check)
}
