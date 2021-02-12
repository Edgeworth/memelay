pub const EP: f64 = 1.0e-6;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
// Only one crossover function will be applied at a time.
pub enum Crossover {
    // Fixed with given rate. Specify the probabilities for each crossover function.
    Fixed(Vec<f64>),
    // Adaptive - uses 1/sqrt(pop size) as learning rate.
    Adaptive,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
// Each mutation function will be applied with the given rate. This is different to crossover,
// which is only applied once.
pub enum Mutation {
    // Fixed with given rate. Specify the probabilities for each mutation function.
    Fixed(Vec<f64>),
    // Adaptive - uses 1/sqrt(pop size) as learning rate.
    Adaptive,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Survival {
    TopProportion(f64),
    SpeciesTopProportion(f64), // Top proportion for each species.
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum Selection {
    Sus,
    Roulette,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum Niching {
    None,
    SharedFitness,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum Species {
    None,
    TargetNumber(usize), // Target number of species.
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Cfg {
    pub pop_size: usize,
    pub crossover: Crossover,
    pub mutation: Mutation, // Mutation rate per bit / basic block.
    pub survival: Survival,
    pub selection: Selection,
    pub niching: Niching,
    pub species: Species,
}

impl Cfg {
    pub fn new(pop_size: usize) -> Self {
        Self {
            pop_size,
            crossover: Crossover::Adaptive,
            mutation: Mutation::Adaptive,
            survival: Survival::TopProportion(0.1),
            selection: Selection::Sus,
            niching: Niching::None,
            species: Species::None,
        }
    }

    pub fn with_pop_size(self, pop_size: usize) -> Self {
        Self { pop_size, ..self }
    }

    pub fn with_crossover(self, crossover: Crossover) -> Self {
        Self { crossover, ..self }
    }

    pub fn with_mutation(self, mutation: Mutation) -> Self {
        Self { mutation, ..self }
    }

    pub fn with_survival(self, survival: Survival) -> Self {
        Self { survival, ..self }
    }

    pub fn with_selection(self, selection: Selection) -> Self {
        Self { selection, ..self }
    }

    pub fn with_niching(self, niching: Niching) -> Self {
        Self { niching, ..self }
    }

    pub fn with_species(self, species: Species) -> Self {
        Self { species, ..self }
    }
}
