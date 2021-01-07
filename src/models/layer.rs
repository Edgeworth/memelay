use crate::prelude::*;
use crate::types::Key;
use derive_more::Display;
use rand::Rng;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Display)]
#[display(fmt = "{:?}", keys)]
pub struct Layer {
    pub keys: Vec<Key>,
}

impl Layer {
    pub fn new(keys: &[Key]) -> Self {
        Self { keys: keys.to_vec() }
    }

    pub fn rand_with_size(len: usize) -> Self {
        let mut r = rand::thread_rng();
        Self { keys: (0..len).map(|_| r.gen()).collect() }
    }

    pub fn num_physical(&self) -> usize {
        self.keys.len()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Layout {
    pub layers: Vec<Layer>,
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.layers.iter().enumerate() {
            writeln!(f, "  Layer {}: {}", i, v)?;
        }
        Ok(())
    }
}

impl Layout {
    pub fn new() -> Self {
        Self { layers: vec![] }
    }

    pub fn with_layer(mut self, l: Layer) -> Self {
        self.layers.push(l);
        self
    }

    pub fn num_physical(&self) -> usize {
        self.layers.get(0).map(|x| x.num_physical()).unwrap_or(0)
    }
}
