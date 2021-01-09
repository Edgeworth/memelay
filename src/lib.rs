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
use structopt::StructOpt;

pub mod constants;
mod fitness;
mod ingest;
mod models;
pub mod prelude;
mod types;

#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    layout: String,
    cost: Vec<f64>,
    fing: Vec<Finger>,
    corpus: Vec<PhysEv>,
}

impl Env {
    pub fn new(layout: String, cost: Vec<f64>, fing: Vec<Finger>, corpus: Vec<PhysEv>) -> Self {
        Self { layout, cost, fing, corpus }
    }

    pub fn format_solution(&self, l: &Layout) -> String {
        let mut s = String::new();
        for (i, layer) in l.layers.iter().enumerate() {
            s += &format!("Layer {}\n", i);
            let mut idx = 0;
            for c in self.layout.chars() {
                if c == 'X' {
                    let mut kstr = format!("{:?}", layer.keys[idx]);
                    kstr.retain(|c| !r"() ".contains(c));
                    let kstr = kstr.replace("EnumSet", "");
                    s += &kstr;
                    idx += 1;
                } else {
                    s.push(c);
                }
            }
            s.push('\n');
        }
        s
    }

    pub fn num_physical(&self) -> usize {
        self.cost.len()
    }
}

impl Envionment for Env {}
impl Default for Env {
    fn default() -> Self {
        Self::new("".to_owned(), vec![], vec![], vec![])
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "hodlr", about = "Hodlr CLI")]
struct Args {
    #[structopt(short, long, default_value = "1000")]
    pop_size: i32,
}

pub fn run() -> Result<()> {
    let args = Args::from_args();
    let species_target = args.pop_size / 10;
    let env = env_from_file("moonlander.cfg", "keys_notime.data.bak")?;
    let (top, _) = Population::<Layout, Env, Fitness>::new()
        .size(args.pop_size)
        .constrain(env.clone())
        .impose(Fitness::new(env.clone()))
        .populate_base()
        .dynamic_distance(true)
        .stagnation(10, vec![Genocide::KillWorst(0.9)])
        .debug(false)
        .survivor_criteria(SurvivalCriteria::Fittest)
        .parental_criteria(ParentalCriteria::BiasedRandom)
        .configure(Config {
            inbreed_rate: 0.001,
            crossover_rate: 0.2,
            distance: 0.5,
            species_target: species_target as usize,
        })
        .run(|model, fit, num| {
            println!("Generation: {} score: {:.3?}", num, fit);
            if num % 10 == 0 {
                println!("{}", env.format_solution(model));
            }
            num == 20000
        })
        .map_err(|e| eyre!(e))?;

    println!("Solution: {}", env.format_solution(&top));

    Ok(())
}
