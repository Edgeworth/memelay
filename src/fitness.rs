use crate::layout::Layout;
use crate::prelude::*;
use crate::types::Finger;
use radiate::Problem;
use std::fs;
use std::path::Path;

pub fn fitness_from_file<P: AsRef<Path>>(p: P) -> Result<Fitness> {
    const ALLOWED: &str = "RLPMIT-.0123456789";
    let file = fs::read_to_string(p)?;
    let mut cost = Vec::new();
    let mut fing = Vec::new();
    for i in file.split(char::is_whitespace) {
        let s: String = i.chars().filter(|&c| ALLOWED.contains(c)).collect();
        if !s.is_empty() {
            println!("s: {}", s);
        }
    }
    if cost.len() != fing.len() {
        Err(eyre!("{} costs does not match {} fingers", cost.len(), fing.len()))
    } else {
        Ok(Fitness::new(cost, fing))
    }
}

pub struct Fitness {
    cost: Vec<f64>,
    fing: Vec<Finger>,
}

impl Fitness {
    pub fn new(cost: Vec<f64>, fing: Vec<Finger>) -> Self {
        Self { cost, fing }
    }
}

impl Problem<Layout> for Fitness {
    fn empty() -> Self {
        Self::new(vec![], vec![])
    }

    fn solve(&self, model: &mut Layout) -> f32 {
        0.0
    }
}
