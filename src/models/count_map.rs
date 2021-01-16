use derive_more::Display;
use std::fmt;
use std::slice::Iter;

use crate::types::KC;

fn press_to_count(press: bool) -> i32 {
    if press {
        1
    } else {
        -1
    }
}

#[derive(Debug, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash, Display)]
#[display(fmt = "{:?}", c)]
pub struct CountMap<T: Copy + PartialEq + Ord + fmt::Debug> {
    c: Vec<(T, i32)>,
}

impl<T: Copy + PartialEq + Ord + fmt::Debug> CountMap<T> {
    pub fn new() -> Self {
        Self { c: Vec::new() }
    }

    pub fn from_vec(mut c: Vec<(T, i32)>) -> Self {
        c.sort();
        c.dedup();
        let c = c.into_iter().filter(|x| x.1 > 0).collect::<Vec<_>>();
        if c.iter().any(|x| x.1 < 0) {
            panic!("can't have negative count");
        }
        Self { c }
    }

    pub fn get_count(&self, k: T) -> i32 {
        self.c.iter().find(|x| x.0 == k).map(|x| x.1).unwrap_or(0)
    }

    pub fn num_pressed(&self) -> usize {
        self.c.len()
    }

    pub fn peek_adjust(&self, k: T, press: bool) -> i32 {
        let count = press_to_count(press);
        self.get_count(k) + count
    }

    pub fn adjust_count(&mut self, k: T, press: bool) -> i32 {
        let count = press_to_count(press);
        if let Some(idx) = self.c.iter_mut().position(|x| x.0 == k) {
            self.c[idx].1 += count;
            if self.c[idx].1 == 0 {
                self.c.remove(idx);
                0
            } else {
                self.c[idx].1
            }
        } else {
            self.c.push((k, count));
            self.c.sort(); // Sort to make sure same instances compare same.
            count
        }
    }

    pub fn iter(&self) -> Iter<'_, (T, i32)> {
        self.c.iter()
    }

    pub fn is_superset(&self, o: &CountMap<T>) -> bool {
        for &(k, v) in o.iter() {
            if self.get_count(k) < v {
                return false;
            }
        }
        true
    }
}

impl CountMap<KC> {
    pub fn mods(&self) -> CountMap<KC> {
        CountMap::from_vec(self.c.iter().copied().filter(|x| x.0.is_mod()).collect())
    }

    pub fn regular(&self) -> CountMap<KC> {
        CountMap::from_vec(self.c.iter().copied().filter(|x| !x.0.is_mod()).collect())
    }
}
