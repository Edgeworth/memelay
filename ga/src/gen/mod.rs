pub mod evaluated;
mod species;
pub mod unevaluated;

// Potentially self-adaptive parameters per state.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Params {
    mutation_rate: f64,
    crossover_rate: f64,
}
