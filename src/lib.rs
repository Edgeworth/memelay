#![warn(rust_2018_idioms, clippy::all)]
#![feature(
    array_chunks,
    array_windows,
    bool_to_option,
    const_fn,
    destructuring_assignment,
    map_first_last,
    option_expect_none,
    option_result_contains,
    option_unwrap_none,
    stmt_expr_attributes,
    trait_alias,
    type_alias_impl_trait
)]

use crate::eval::LayoutEval;
use crate::ingest::{load_layouts, load_params};
use crate::layout::Layout;
use eyre::Result;
use memega::cfg::{Cfg, Crossover, Duplicates, Mutation, Niching, Species, Stagnation, Survival};
use memega::hyper::HyperBuilder;
use memega::runner::Runner;
use memega::{CachedEvaluator, Evaluator};
use rand::prelude::SliceRandom;
use std::path::{Path, PathBuf};
use std::time::Duration;
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
        default_value = "data/unigrams.data",
        parse(from_os_str),
        help = "Data file describing unigrams"
    )]
    pub unigrams_path: PathBuf,

    #[structopt(
        long,
        default_value = "data/bigrams.data",
        parse(from_os_str),
        help = "Data file describing bigrams"
    )]
    pub bigrams_path: PathBuf,

    #[structopt(short, long, parse(from_os_str), help = "Evaluate a given layout")]
    pub eval_layout: Option<PathBuf>,
}

pub fn eval_layout<P: AsRef<Path>>(p: P) -> Result<()> {
    let args = Args::from_args();
    let eval = LayoutEval::from_args(&args)?;
    let l = load_layouts(p)?;
    let fitness = eval.fitness(&l[0]);
    println!("layout:\n{}", eval.params.format(&l[0]));
    println!("fitness: {}", fitness);
    Ok(())
}

pub fn layout_runner(cfg: Cfg) -> Result<Runner<CachedEvaluator<LayoutEval>>> {
    let args = Args::from_args();
    let eval = LayoutEval::from_args(&args)?;
    let initial_keys = load_layouts("data/default.layout")?;
    let keyset = initial_keys[0].keys.clone();
    println!("Seeded with {} layouts", initial_keys.len());
    Ok(Runner::from_initial(CachedEvaluator::new(eval, 1000), cfg, initial_keys, move || {
        let mut keys = keyset.clone();
        keys.shuffle(&mut rand::thread_rng());
        Layout::new(keys)
    }))
}

pub fn evolve(cfg: Cfg) -> Result<()> {
    let args = Args::from_args();
    let params = load_params(&args.params_path)?;
    let mut runner = layout_runner(cfg)?;

    for i in 0..200001 {
        let mut r = runner.run_iter()?;
        if i % 50 == 0 {
            println!("Generation {}: {}", i + 1, r.gen.nth(0).base_fitness);
            println!("{}", runner.summary(&mut r));
            println!("{}", runner.summary_sample(&mut r, 6, |v| params.format(v)));
        }
    }

    Ok(())
}

pub fn hyper_evolve() -> Result<()> {
    let mut builder = HyperBuilder::new(100, Duration::from_millis(2000));
    builder.add(1.0, &|cfg| layout_runner(cfg).unwrap());
    let mut runner = builder.build();

    for i in 0..10001 {
        let mut r = runner.run_iter()?;
        println!("Generation {}: {}", i + 1, r.gen.nth(0).base_fitness);
        if i % 10 == 0 {
            println!("{}", runner.summary(&mut r));
            println!("{}", runner.summary_sample(&mut r, 5, |v| format!("{:?}", v)));
        }
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let args = Args::from_args();
    // Remember to update these values if add more mutation/crossover strategies.
    let cfg = Cfg::new(1000)
        .with_mutation(Mutation::Fixed(vec![0.001, 0.2, 0.2, 0.2, 0.2]))
        .with_crossover(Crossover::Fixed(vec![0.3, 0.1, 0.2, 0.7]))
        .with_survival(Survival::SpeciesTopProportion(0.1))
        .with_species(Species::TargetNumber(100))
        .with_niching(Niching::None)
        .with_stagnation(Stagnation::None)
        .with_duplicates(Duplicates::DisallowDuplicates)
        .with_par_fitness(true)
        .with_par_dist(true);

    if let Some(p) = args.eval_layout {
        eval_layout(p)?;
    } else {
        evolve(cfg)?;
        // hyper_evolve()?;
    }

    Ok(())
}
