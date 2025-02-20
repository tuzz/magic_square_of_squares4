use crate::hashing::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::mem::swap;

#[derive(Default)]
pub struct MagicSums {
    elements: Vec<Element>,
    lookup: NoHashMap<u64, usize>,
    tmp: Vec<usize>,
}

struct Element {
    number: u64,
    is_square: bool,
    partners: Vec<usize>,
}

impl MagicSums {
    pub fn get_or_insert_element<F, G>(&mut self, number: u64, is_square_fn: F, condition_fn: G) -> Option<(usize, bool)>
        where F: Fn() -> bool,
              G: Fn(bool) -> bool
    {
        match self.lookup.entry(hash(number)) {
            Occupied(entry) => {
                let index = *entry.get();
                let is_square = self.elements[index].is_square;

                Some((index, is_square))
            },
            Vacant(entry) => {
                let is_square = is_square_fn();

                if condition_fn(is_square) {
                    let index = self.elements.len();

                    self.elements.push(Element { number, is_square, partners: vec![] });
                    entry.insert(index);

                    Some((index, is_square))
                } else {
                    None
                }
            },
        }
    }

    pub fn add_partnership(&mut self, index1: usize, index2: usize, index3: usize) {
        self.elements[index1].partners.extend([index2, index3]);
        self.elements[index2].partners.extend([index1, index3]);
        self.elements[index3].partners.extend([index1, index2]);
    }

    pub fn remove_if_less_than_two_partners(&mut self) {
        loop {
            let mut removed = false;

            for i in 0..self.elements.len() {
                let element = &mut self.elements[i];

                if element.partners.len() == 1 {
                    self.lookup.remove(&hash(element.number));
                    swap(&mut element.partners, &mut self.tmp);

                    for j in self.tmp.drain(..) {
                        self.elements[j].partners.retain(|&k| k != i);
                    }

                    removed = true;
                }
            }

            if !removed { break; }
        }
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.lookup.clear();
    }
}
