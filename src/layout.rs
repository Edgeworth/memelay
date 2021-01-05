use crate::Env;
use radiate::Genome;
use rand::Rng;
use std::sync::{Arc, RwLock};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Finger {
    LP,
    LR,
    LM,
    LI,
    LT,
    RP,
    RR,
    RM,
    RI,
    RT,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Layout {
    pub data: Vec<char>,
}

impl Layout {
    pub fn new(alph: &[char]) -> Self {
        let mut r = rand::thread_rng();
        Layout { data: (0..12).map(|_| alph[r.gen_range(0, alph.len())]).collect() }
    }

    pub fn as_string(&self) -> String {
        self.data.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("")
    }
}

impl Genome<Layout, Env> for Layout {
    fn crossover(
        parent_one: &Layout,
        parent_two: &Layout,
        env: Arc<RwLock<Env>>,
        crossover_rate: f32,
    ) -> Option<Layout> {
        let params = env.read().unwrap();
        let mut r = rand::thread_rng();
        let mut new_data = Vec::new();

        if r.gen::<f32>() < crossover_rate {
            for (one, two) in parent_one.data.iter().zip(parent_two.data.iter()) {
                if one != two {
                    new_data.push(*one);
                } else {
                    new_data.push(*two);
                }
            }
        } else {
            new_data = parent_one.data.clone();
            let swap_index = r.gen_range(0, new_data.len());
            new_data[swap_index] = params.alph[r.gen_range(0, params.alph.len())];
        }
        Some(Layout { data: new_data })
    }

    fn distance(one: &Layout, two: &Layout, _: Arc<RwLock<Env>>) -> f32 {
        let mut total = 0_f32;
        for (i, j) in one.data.iter().zip(two.data.iter()) {
            if i == j {
                total += 1_f32;
            }
        }
        one.data.len() as f32 / total
    }

    fn base(env: &mut Env) -> Layout {
        Layout::new(&env.alph)
    }
}
