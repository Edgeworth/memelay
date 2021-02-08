pub mod evaluated;
mod species;
pub mod unevaluated;

// Potentially self-adaptive parameters per state.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Params {
    // Conventionally, the first element will be the weight of doing no mutation or crossover.
    mutation_rates: Vec<f64>,
    crossover_rates: Vec<f64>,
}
