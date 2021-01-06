#![warn(rust_2018_idioms, clippy::all)]
#![feature(
    const_fn,
    option_result_contains,
    trait_alias,
    iterator_fold_self,
    type_alias_impl_trait,
    partition_point,
    bool_to_option
)]

use crate::fitness::{fitness_from_file, Fitness};
use crate::layout::Layout;
use crate::prelude::*;
use radiate::{Config, Envionment, Genocide, ParentalCriteria, Population, SurvivalCriteria};

mod fitness;
mod layout;
pub mod prelude;
mod types;

#[derive(Debug, Clone)]
pub struct Env {}

impl Env {
    pub fn new() -> Self {
        Self {}
    }
}

impl Envionment for Env {}
impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

pub fn run() -> Result<()> {
    fitness_from_file("moonlander.cfg")?;
    let (top, _) = Population::<Layout, Env, Fitness>::new()
        .impose(fitness_from_file("moonlander.cfg")?)
        .size(100)
        .populate_base()
        .dynamic_distance(true)
        .stagnation(10, vec![Genocide::KillWorst(0.9)])
        .debug(false)
        .survivor_criteria(SurvivalCriteria::Fittest)
        .parental_criteria(ParentalCriteria::BiasedRandom)
        .configure(Config {
            inbreed_rate: 0.001,
            crossover_rate: 0.75,
            distance: 0.5,
            species_target: 5,
        })
        .run(|model, fit, num| {
            println!("Generation: {} score: {:.3?}\t{:?}", num, fit, model.to_string());
            fit == 12.0 || num == 500
        })
        .map_err(|e| eyre!(e))?;

    println!("Solution: {:?}", top.to_string());

    Ok(())
}
