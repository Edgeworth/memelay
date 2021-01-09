#![warn(rust_2018_idioms, clippy::all)]
#![feature(
    const_fn,
    option_result_contains,
    trait_alias,
    iterator_fold_self,
    type_alias_impl_trait,
    partition_point,
    bool_to_option,
    map_first_last
)]

use crate::fitness::Fitness;
use crate::ingest::env_from_file;
use crate::models::layer::Layout;
use crate::prelude::*;
use crate::types::{Finger, PhysEv};
use radiate::{Config, Envionment, Genocide, ParentalCriteria, Population, SurvivalCriteria};

mod fitness;
mod ingest;
mod models;
pub mod prelude;
mod types;

#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    cost: Vec<f64>,
    fing: Vec<Finger>,
    corpus: Vec<PhysEv>,
}

impl Env {
    pub fn new(cost: Vec<f64>, fing: Vec<Finger>, corpus: Vec<PhysEv>) -> Self {
        Self { cost, fing, corpus }
    }
}

impl Envionment for Env {}
impl Default for Env {
    fn default() -> Self {
        Self::new(vec![], vec![], vec![])
    }
}

pub fn run() -> Result<()> {
    let env = env_from_file("moonlander.cfg", "keys_notime.data")?;
    let (top, _) = Population::<Layout, Env, Fitness>::new()
        .constrain(env.clone())
        .impose(Fitness::new(env))
        .size(1)
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
        .run(|_, fit, num| {
            println!("Generation: {} score: {:.3?}", num, fit);
            num == 2
        })
        .map_err(|e| eyre!(e))?;

    println!("Solution: {}", top);

    Ok(())
}
