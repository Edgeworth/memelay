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

use crate::env::Env;
use crate::fitness::Fitness;
use crate::models::layout::Layout;
use crate::prelude::*;
use env::Constants;
use radiate::{Config, Genocide, ParentalCriteria, Population, Problem, SurvivalCriteria};
use std::path::PathBuf;
use structopt::StructOpt;

mod env;
mod fitness;
mod genome;
mod ingest;
mod models;
pub mod prelude;
mod types;

#[derive(Debug, StructOpt)]
#[structopt(name = "hodlr", about = "Hodlr CLI")]
pub struct Args {
    #[structopt(
        long,
        default_value = "moonlander.cfg",
        parse(from_os_str),
        help = "Config file describing target layout and costs"
    )]
    cfg_path: PathBuf,

    #[structopt(
        long,
        parse(from_os_str),
        help = "Corpus file describing typing data to optimise to"
    )]
    corpus_path: PathBuf,

    #[structopt(flatten)]
    cnst: Constants,
}

pub fn run() -> Result<()> {
    let env = Env::from_args(Args::from_args())?;
    let species_target = env.cnst.pop_size / 10;

    // let mut layout = Layout::base(&mut env);
    // println!("Rand layout: {}", env.layout_cfg.format_solution(&layout));

    // return Ok(());

    let mut pop = Population::<Layout, Env, Fitness>::new()
        .size(env.cnst.pop_size as i32)
        .constrain(env.clone())
        .impose(Fitness::new(env.clone()))
        .populate_base()
        .dynamic_distance(true)
        .stagnation(10, vec![Genocide::KillWorst(0.9)])
        .debug(env.cnst.debug)
        .survivor_criteria(SurvivalCriteria::Fittest)
        .parental_criteria(ParentalCriteria::BiasedRandom)
        .configure(Config {
            inbreed_rate: 0.001,
            crossover_rate: 0.2,
            distance: 0.5,
            species_target: species_target as usize,
        });

    let (top, _) = pop
        .run(|model, fit, num| {
            println!("Generation: {} score: {:.3?}", num, fit);
            if num % 10 == 0 {
                println!("{}", env.layout_cfg.format_solution(model));
            }
            num == env.cnst.runs as i32
        })
        .map_err(|e| eyre!(e))?;

    let fitness = Fitness::new(env.clone());
    for (idx, mem) in pop.members_mut().iter().take(10).enumerate() {
        let mut l = mem.member.write().unwrap();
        let val = fitness.solve(&mut l);
        let fmt = env.layout_cfg.format_solution(&l);
        println!("Soln {} fitness {}: {}", idx, val, fmt);
    }

    println!("Solution: {}", env.layout_cfg.format_solution(&top));

    Ok(())
}
