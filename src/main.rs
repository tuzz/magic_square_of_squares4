use fxhash::FxHashMap;
use std::collections::VecDeque;

fn main() {
    let mut squares_by_class = [vec![], vec![], vec![]];
    let mut sums_by_class = [FxHashMap::default(), FxHashMap::default(), FxHashMap::default()];
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

            let Some(squares) = sums.get(&sum) else { continue };

            // println!("{} = {} + ... {:?}", center_square * 3, center_square, squares);
        }

        let square_class = (square % 72 / 24) as usize;
        let center_sum = square + square;
        let center_sum_class = (center_sum % 72 / 24) as usize;

        sums_by_class[center_sum_class].insert(center_sum, vec![]);
        sums_to_check.push_back(center_sum);

        for (i, squares) in squares_by_class.iter().enumerate() {
            let sum_class = (square_class + i) % 3;
            let sums = &mut sums_by_class[sum_class];

            for &square2 in squares {
                if let Some(vec) = sums.get_mut(&(square + square2)) {
                    vec.push(square);
                }
            }
        }

        squares_by_class[square_class].push(square);
    }
}
