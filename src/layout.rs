use crate::types::Kc;
use derive_more::Display;
use memega::ops::mutation::mutate_gen;

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Display)]
#[display(fmt = "{:?}", keys)]
pub struct Layout {
    pub keys: Vec<Kc>,
}

impl Layout {
    pub fn new(keys: Vec<Kc>) -> Self {
        Self { keys }
    }

    pub fn rand_with_size(len: usize) -> Self {
        Self { keys: (0..len).map(|_| mutate_gen::<Kc>()).collect() }
    }

    pub fn num_physical(&self) -> usize {
        self.keys.len()
    }
}
