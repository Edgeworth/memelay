use crate::runner::SelectionMethod;
use crate::runner::SelectionMethod::StochasticUniformSampling;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cfg {
    pub crossover_rate: f64,
    pub pop_size: usize,
    pub top_prop: f64,
    pub selection_method: SelectionMethod,
}

impl Cfg {
    pub fn new(pop_size: usize) -> Self {
        Self {
            crossover_rate: 0.3,
            pop_size,
            top_prop: 0.1,
            selection_method: StochasticUniformSampling,
        }
    }

    pub fn with_selection_method(self, selection_method: SelectionMethod) -> Self {
        Self { selection_method, ..self }
    }
}
