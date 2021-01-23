use crate::generation::Selection;
use crate::niching::Niching;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cfg {
    pub crossover_rate: f64,
    pub mutation_rate: f64, // Mutation rate per bit / basic block.
    pub pop_size: usize,
    pub top_prop: f64,
    pub selection: Selection,
    pub niching: Niching,
}

impl Cfg {
    pub fn new(pop_size: usize) -> Self {
        Self {
            crossover_rate: 0.7,
            mutation_rate: 0.1,
            pop_size,
            top_prop: 0.1,
            selection: Selection::Sus,
            niching: Niching::None,
        }
    }

    pub fn with_pop_size(self, pop_size: usize) -> Self {
        Self { pop_size, ..self }
    }

    pub fn with_crossover_rate(self, crossover_rate: f64) -> Self {
        Self { crossover_rate, ..self }
    }

    pub fn with_mutation_rate(self, mutation_rate: f64) -> Self {
        Self { mutation_rate, ..self }
    }

    pub fn with_selection(self, selection: Selection) -> Self {
        Self { selection, ..self }
    }

    pub fn with_niching(self, niching: Niching) -> Self {
        Self { niching, ..self }
    }
}
