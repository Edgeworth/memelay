use crate::types::KeyCode;
use crate::Env;
use radiate::Genome;
use rand::seq::IteratorRandom;
use rand::Rng;
use std::sync::{Arc, RwLock};
use strum::IntoEnumIterator;

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    keys: Vec<KeyCode>,
}

impl Layer {
    pub fn new(len: usize) -> Self {
        let mut r = rand::thread_rng();
        Self { keys: (0..len).map(|_| KeyCode::iter().choose(&mut r).unwrap()).collect() }
    }

    pub fn to_string(&self) -> String {
        "test".to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    layers: Vec<Layer>,
}

impl Layout {
    pub fn new() -> Self {
        Self { layers: vec![] }
    }

    pub fn to_string(&self) -> String {
        self.layers
            .iter()
            .enumerate()
            .map(|(i, v)| format!("  Layer {}: {}\n", i, v.to_string()))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl Genome<Layout, Env> for Layout {
    fn crossover(
        parent_one: &Layout,
        parent_two: &Layout,
        env: Arc<RwLock<Env>>,
        crossover_rate: f32,
    ) -> Option<Layout> {
        Some(parent_one.clone())
    }

    fn distance(one: &Layout, two: &Layout, _: Arc<RwLock<Env>>) -> f32 {
        1.0
    }

    fn base(env: &mut Env) -> Layout {
        Layout::new()
    }
}
