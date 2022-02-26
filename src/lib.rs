#![warn(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    nonstandard_style,
    noop_method_call,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    trivial_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused,
    variant_size_differences
)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::items_after_statements,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

use std::path::{Path, PathBuf};
use std::time::Duration;

use clap::Parser;
use eyre::Result;
use memega::cfg::{
    Cfg, Crossover, Duplicates, Mutation, Niching, Replacement, Species, Stagnation, Survival,
};
use memega::eval::{CachedEvaluator, Evaluator};
use memega::hyper::builder::HyperBuilder;
use memega::run::multirun::multirun;
use memega::run::runner::Runner;
use rand::prelude::SliceRandom;

use crate::eval::LayoutEval;
use crate::ingest::{load_model, load_seeds};

pub mod eval;
pub mod ingest;
pub mod model;
pub mod types;

#[derive(Debug, Parser)]
#[clap(name = "hodlr", about = "Hodlr CLI")]
pub struct Args {
    #[clap(
        long,
        default_value = "cfg/layer0.cfg",
        parse(from_os_str),
        help = "Config file describing the model"
    )]
    pub model_path: PathBuf,

    #[clap(long, parse(from_os_str), help = "Config file describing seed layouts")]
    pub seed_path: Option<PathBuf>,

    #[clap(
        long,
        default_value = "data/unigrams.data",
        parse(from_os_str),
        help = "Data file describing unigrams"
    )]
    pub unigrams_path: PathBuf,

    #[clap(
        long,
        default_value = "data/bigrams.data",
        parse(from_os_str),
        help = "Data file describing bigrams"
    )]
    pub bigrams_path: PathBuf,

    #[clap(
        long,
        default_value = "data/trigrams.data",
        parse(from_os_str),
        help = "Data file describing trigrams"
    )]
    pub trigrams_path: PathBuf,

    #[clap(short, long, parse(from_os_str), help = "Evaluate a given layout")]
    pub eval_layout: Option<PathBuf>,
}

pub fn eval_layout<P: AsRef<Path>>(p: P) -> Result<()> {
    let args = Args::parse();
    let eval = LayoutEval::from_args(&args)?;
    let l = load_seeds(p)?;
    let fitness = eval.fitness(&l[0], 0);
    println!("layout:\n{}", eval.model.format(&l[0]));
    println!("fitness: {}", fitness);
    Ok(())
}

pub fn layout_runner(cfg: Cfg) -> Result<Runner<CachedEvaluator<LayoutEval>>> {
    let args = Args::parse();
    let model = load_model(&args.model_path)?;
    let eval = CachedEvaluator::new(LayoutEval::from_args(&args)?, 1000);
    let genfn = move || {
        let mut keys = model.without_fixed(&model.universe);
        keys.shuffle(&mut rand::thread_rng());
        model.with_fixed(&keys)
    };
    if let Some(seed) = args.seed_path {
        let initial_keys = load_seeds(seed)?;
        Ok(Runner::from_initial(eval, cfg, initial_keys, genfn))
    } else {
        Ok(Runner::new(eval, cfg, genfn))
    }
}

pub fn evolve(cfg: Cfg) -> Result<()> {
    let args = Args::parse();
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

pub fn multi_evolve(cfg: &Cfg) -> Result<()> {
    let args = Args::parse();
    let model = load_model(&args.model_path)?;
    let mut results = multirun(20, 5000, cfg, |cfg| layout_runner(cfg).unwrap());

    results.sort_unstable_by(|(_, r1), (_, r2)| {
        r2.nth(0).base_fitness.partial_cmp(&r1.nth(0).base_fitness).unwrap()
    });

    for (runner, r) in results.iter_mut().take(10) {
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
    let args = Args::parse();
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
