use crate::cfg::{Cfg, Crossover, Mutation, Niching, Species, Survival};

pub mod knapsack;
pub mod rastrigin;
pub mod target_string;
pub mod func;

pub fn all_cfg() -> Cfg {
    Cfg::new(100)
        .with_mutation(Mutation::Adaptive)
        .with_crossover(Crossover::Adaptive)
        .with_survival(Survival::SpeciesTopProportion(0.1))
        .with_species(Species::TargetNumber(10))
        .with_niching(Niching::SharedFitness)
}

pub fn none_cfg() -> Cfg {
    Cfg::new(100)
}
