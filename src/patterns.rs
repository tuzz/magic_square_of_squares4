use crate::hashing::*;
use crate::shared_vec::*;

pub struct Cell {
    pub value: u64,
    pub magic_sum_minus_value: u64
}

impl Cell {
    pub fn new(value: u64, magic_sum: u64) -> Self {
        Self { value, magic_sum_minus_value: magic_sum - value }
    }
}

// The patterns are from figure 5 of http://www.multimagie.com/Search.pdf#page=2

pub fn check_pattern_2(squares: &NoHashSet<u64>, center: &Cell, top_left: &Cell, bottom_right: &Cell, top_right: &Cell, bottom_left: &Cell) {
    let Some(top_middle) = top_left.magic_sum_minus_value.checked_sub(top_right.value) else { return };
    let Some(middle_left) = top_left.magic_sum_minus_value.checked_sub(bottom_left.value) else { return };
    let bottom_middle = bottom_left.magic_sum_minus_value - bottom_right.value;

    let mut num_squares = 5;
    if squares.contains(&top_middle) { num_squares += 1; }
    if squares.contains(&middle_left) { num_squares += 1; }
    if squares.contains(&bottom_middle) { num_squares += 1; }
    if num_squares < 6 { return; }

    let middle_right = center.magic_sum_minus_value - middle_left;
    if squares.contains(&middle_right) { num_squares += 1; }
    if num_squares < 7 { return; }

    println!("{} | {} | {}", top_left.value, top_middle, top_right.value);
    println!("{} | {} | {}", middle_left, center.value, middle_right);
    println!("{} | {} | {}\n", bottom_left.value, bottom_middle, bottom_right.value);
}

pub fn check_pattern_5(sums_by_class: &[NoHashMap<u64, SharedVec>; 3], center: &Cell, top_middle: &Cell, bottom_middle: &Cell, middle_left: &Cell, middle_right: &Cell) {
    for sums in sums_by_class {
        if let Some(numbers) = sums.get(&hash(top_middle.magic_sum_minus_value)) {
            let numbers = numbers.0.lock().unwrap();

            for &number in numbers.iter() {
                let top_left = number as u64 * number as u64;
                let top_right = top_middle.magic_sum_minus_value - top_left;

                let Some(bottom_left) = middle_left.magic_sum_minus_value.checked_sub(top_left) else { continue };
                let Some(bottom_right) = middle_right.magic_sum_minus_value.checked_sub(top_right) else { continue };
                if bottom_left + bottom_right != bottom_middle.magic_sum_minus_value { continue };
                if top_left + bottom_right != center.magic_sum_minus_value { continue; }
                if top_right + bottom_left != center.magic_sum_minus_value { continue; }

                println!("{} | {} | {}", top_left, top_middle.value, top_right);
                println!("{} | {} | {}", middle_left.value, center.value, middle_right.value);
                println!("{} | {} | {}\n", bottom_left, bottom_middle.value, bottom_right);
            }
        }
    }
}

pub fn check_patterns_3_4_and_6(squares: &NoHashSet<u64>, center: &Cell, top_right: &Cell, bottom_left: &Cell, middle_left: &Cell, middle_right: &Cell) {
    let Some(top_left) = bottom_left.magic_sum_minus_value.checked_sub(middle_left.value) else { return };
    let Some(bottom_right) = top_right.magic_sum_minus_value.checked_sub(middle_right.value) else { return };
    let Some(top_middle) = top_right.magic_sum_minus_value.checked_sub(top_left) else { return };

    let mut num_squares = 5;
    if squares.contains(&top_left) { num_squares += 1; }
    if squares.contains(&bottom_right) { num_squares += 1; }
    if squares.contains(&top_middle) { num_squares += 1; }
    if num_squares < 6 { return; }

    let bottom_middle = bottom_left.magic_sum_minus_value - bottom_right;
    if squares.contains(&bottom_middle) { num_squares += 1; }
    if num_squares < 7 { return; }

    if crate::HIDE_KNOWN_SOLUTION {
        let k2 = center.value / 180_625;
        let k = k2.isqrt();
        if k * k == k2 { return; }
    }

    println!("{} | {} | {}", top_left, top_middle, top_right.value);
    println!("{} | {} | {}", middle_left.value, center.value, middle_right.value);
    println!("{} | {} | {}\n", bottom_left.value, bottom_middle, bottom_right);
}
