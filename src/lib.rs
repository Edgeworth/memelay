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
use crate::ingest::{load_model, load_seeds};
use crate::layout::Layout;
use eyre::Result;
use memega::cfg::{
    Cfg, Crossover, Duplicates, Mutation, Niching, Replacement, Species, Stagnation, Survival,
};
use memega::eval::{CachedEvaluator, Evaluator};
use memega::hyper::HyperBuilder;
use memega::multirun::multirun;
use memega::runner::Runner;
use rand::prelude::SliceRandom;
use std::path::{Path, PathBuf};
use std::time::Duration;
use structopt::StructOpt;

pub mod eval;
pub mod ingest;
pub mod layout;
pub mod model;
pub mod types;

#[derive(Debug, StructOpt)]
#[structopt(name = "hodlr", about = "Hodlr CLI")]
pub struct Args {
    #[structopt(
        long,
        default_value = "cfg/layer0.cfg",
        parse(from_os_str),
        help = "Config file describing the model"
    )]
    pub model_path: PathBuf,

    #[structopt(long, parse(from_os_str), help = "Config file describing seed layouts")]
    pub seed_path: Option<PathBuf>,

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
    let l = load_seeds(p)?;
    let fitness = eval.fitness(&l[0]);
    println!("layout:\n{}", eval.model.format(&l[0]));
    println!("fitness: {}", fitness);
    Ok(())
}

pub fn layout_runner(cfg: Cfg) -> Result<Runner<CachedEvaluator<LayoutEval>>> {
    let args = Args::from_args();
    let model = load_model(&args.model_path)?;
    let eval = CachedEvaluator::new(LayoutEval::from_args(&args)?, 1000);
    let genfn = move || {
        let mut keys = model.without_fixed(&model.keys);
        keys.shuffle(&mut rand::thread_rng());
        Layout::new(model.with_fixed(&keys))
    };
    if let Some(seed) = args.seed_path {
        let initial_keys = load_seeds(seed)?;
        Ok(Runner::from_initial(eval, cfg, initial_keys, genfn))
    } else {
        Ok(Runner::new(eval, cfg, genfn))
    }
}

pub fn evolve(cfg: Cfg) -> Result<()> {
    let args = Args::from_args();
    let model = load_model(&args.model_path)?;
    let mut runner = layout_runner(cfg)?;

    for i in 0..200001 {
        let mut r = runner.run_iter()?;
        if i % 50 == 0 {
            println!("Generation {}: {}", i + 1, r.nth(0).base_fitness);
            println!("{}", runner.summary(&mut r));
            println!("{}", runner.summary_sample(&mut r, 6, |v| model.format(v)));
        }
    }

    Ok(())
}

pub fn multi_evolve(cfg: Cfg) -> Result<()> {
    let args = Args::from_args();
    let model = load_model(&args.model_path)?;
    let mut results = multirun(10, 15000, &cfg, |cfg| layout_runner(cfg).unwrap());

    for (runner, r) in results.iter_mut() {
        println!("{}", runner.summary_sample(r, 1, |v| model.format(v)));
    }

    Ok(())
}

pub fn hyper_evolve() -> Result<()> {
    let mut builder = HyperBuilder::new(100, Duration::from_millis(2000));
    builder.add(1.0, &|cfg| layout_runner(cfg).unwrap());
    let mut runner = builder.build();

    for i in 0..10001 {
        let mut r = runner.run_iter()?;
        println!("Generation {}: {}", i + 1, r.nth(0).base_fitness);
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
        .with_mutation(Mutation::Adaptive)
        .with_crossover(Crossover::Adaptive)
        // .with_mutation(Mutation::Fixed(vec![0.001, 0.2, 0.2, 0.2, 0.2]))
        // .with_crossover(Crossover::Fixed(vec![0.3, 0.1, 0.2, 0.7]))
        .with_survival(Survival::SpeciesTopProportion(0.1))
        .with_species(Species::TargetNumber(100))
        .with_niching(Niching::None)
        .with_stagnation(Stagnation::ContinuousAfter(200))
        .with_replacement(Replacement::ReplaceChildren(0.5))
        .with_duplicates(Duplicates::DisallowDuplicates)
        .with_par_fitness(true)
        .with_par_dist(true);

    if let Some(p) = args.eval_layout {
        eval_layout(p)?;
    } else {
        // multi_evolve(cfg)?;
        evolve(cfg)?;
        // hyper_evolve()?;
    }

    Ok(())
}
