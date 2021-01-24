use crate::cfg::{Cfg, Crossover, Mutation, Niching, Species, Survival};

pub mod knapsack;
pub mod target_string;

pub fn all_cfg() -> Cfg {
    Cfg::new(100)
        .with_mutation(Mutation::Adaptive(1.0 / 10.0))
        .with_crossover(Crossover::Adaptive(1.0 / 10.0))
        .with_survival(Survival::SpeciesTopProportion(0.1))
        .with_species(Species::TargetNumber(10))
        .with_niching(Niching::SharedFitness)
}

pub fn none_cfg() -> Cfg {
    // TODO: undo
    Cfg::new(100)
        .with_mutation(Mutation::Adaptive(1.0 / 10.0))
        .with_crossover(Crossover::Adaptive(1.0 / 10.0))
}
