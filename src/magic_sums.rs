use std::collections::HashMap;
use std::mem::swap;

#[derive(Default)]
pub struct MagicSums {
    elements: Vec<Element>,
    lookup: HashMap<u64, usize>,
    tmp: Vec<usize>,
}

struct Element {
    number: u64,
    is_square: bool,
    partners: Vec<usize>,
}

impl MagicSums {
    pub fn get_or_insert_element(&mut self, number: u64, is_square: bool) -> usize {
        *self.lookup.entry(number).or_insert_with(|| {
            self.elements.push(Element { number, is_square, partners: vec![] });
            self.elements.len() - 1
        })
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

                if element.partners.len() < 2 {
                    self.lookup.remove(&element.number);
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
}
