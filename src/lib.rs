#![warn(rust_2018_idioms, clippy::all)]
#![feature(
    array_chunks,
    array_windows,
    bool_to_option,
    const_fn,
    destructuring_assignment,
    map_first_last,
    option_result_contains,
    option_unwrap_none,
    stmt_expr_attributes,
    trait_alias,
    type_alias_impl_trait
)]

use crate::eval::LayoutEval;
use crate::ingest::load_layout;
use crate::layout::Layout;
use eyre::Result;
use memega::cfg::{Cfg, Crossover, Mutation, Niching, Species, Stagnation, Survival};
use memega::runner::Runner;
use memega::{CachedEvaluator, Evaluator};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

pub mod eval;
pub mod ingest;
pub mod layout;
pub mod types;

#[derive(Debug, StructOpt)]
#[structopt(name = "hodlr", about = "Hodlr CLI")]
pub struct Args {
    #[structopt(
        long,
        default_value = "data/params.cfg",
        parse(from_os_str),
        help = "Config file describing target layout and costs"
    )]
    pub params_path: PathBuf,

    #[structopt(
        long,
        default_value = "data/keys.data",
        parse(from_os_str),
        help = "Data file describing typing data to optimise to"
    )]
    pub data_path: PathBuf,

    #[structopt(short, long, parse(from_os_str), help = "Evaluate a given layout")]
    pub eval_layout: Option<PathBuf>,
}

pub fn eval_layout<P: AsRef<Path>>(eval: LayoutEval, p: P) -> Result<()> {
    let l = load_layout(p)?;
    let fitness = eval.fitness(&l);
    println!("layout: {}", eval.params.format(&l));
    println!("fitness: {}", fitness);
    Ok(())
}

pub fn evolve(eval: LayoutEval, cfg: Cfg) -> Result<()> {
    let initial = load_layout("data/default.layout")?;
    let mut runner = Runner::new(CachedEvaluator::new(eval.clone(), 1000), cfg, move || {
        Layout::rand_with_size(initial.size())
    });

    for i in 0..10000 {
        let mut r = runner.run_iter()?;
        println!("Generation {}: {}", i + 1, r.gen.best().base_fitness);
        if i % 10 == 0 {
            println!("{}", runner.summary(&mut r));
            println!("{}", eval.params.format(&r.gen.best().state.0));
        }
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let args = Args::from_args();
    let eval = LayoutEval::from_args(&args)?;
    // Remember to update these values if add more mutation/crossover strategies.
    let cfg = Cfg::new(100)
        .with_mutation(Mutation::Adaptive)
        .with_crossover(Crossover::Adaptive)
        // .with_survival(Survival::TopProportion(0.2))
        // .with_species(Species::None)
        .with_survival(Survival::SpeciesTopProportion(0.2))
        .with_species(Species::TargetNumber(10))
        .with_niching(Niching::None)
        .with_stagnation(Stagnation::NumGenerations(50))
        .with_par_fitness(true);

    if let Some(p) = args.eval_layout {
        eval_layout(eval, p)?;
    } else {
        evolve(eval, cfg)?;
    }

    Ok(())
}
