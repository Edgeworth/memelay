pub const EP: f64 = 1.0e-6;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Survival {
    TopProportion(f64),
    SpeciesTopProportion(f64), // Top proportion for each species.
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Selection {
    Sus,
    Roulette,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Niching {
    None,
    SharedFitness,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Species {
    None,
    TargetNumber(usize), // Target number of species.
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cfg {
    pub crossover_rate: f64,
    pub mutation_rate: f64, // Mutation rate per bit / basic block.
    pub pop_size: usize,
    pub survival: Survival,
    pub selection: Selection,
    pub niching: Niching,
    pub species: Species,
}

impl Cfg {
    pub fn new(pop_size: usize) -> Self {
        Self {
            crossover_rate: 0.7,
            mutation_rate: 0.1,
            pop_size,
            survival: Survival::TopProportion(0.1),
            selection: Selection::Sus,
            niching: Niching::None,
            species: Species::None,
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
