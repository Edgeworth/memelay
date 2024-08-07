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

use clap::Parser;
use eyre::Result;
use memega::eval::{CachedEvaluator, Evaluator};
use memega::evolve::cfg::{
    Crossover, Duplicates, EvolveCfg, Mutation, Niching, Replacement, Species, Stagnation, Survival,
};
use memega::evolve::evolver::Evolver;
use memega::train::cfg::{Termination, TrainerCfg};
use memega::train::sampler::EmptyDataSampler;
use memega::train::trainer::Trainer;
use rand::prelude::SliceRandom;

use crate::eval::{KeyState, LayoutEval};
use crate::ingest::{load_model, load_seeds};

pub mod eval;
pub mod ingest;
pub mod model;
pub mod types;

#[must_use]
#[derive(Debug, Parser)]
#[clap(name = "hodlr", about = "Hodlr CLI")]
pub struct Args {
    #[clap(
        long,
        default_value = "cfg/layer0.cfg",
        value_name = "FILE",
        value_hint = clap::ValueHint::FilePath,
        help = "Config file describing the model"
    )]
    pub model_path: PathBuf,

    #[clap(
        long,
        value_name = "FILE",
        value_hint = clap::ValueHint::FilePath,
        help = "Config file describing seed layouts"
    )]
    pub seed_path: Option<PathBuf>,

    #[clap(
        long,
        default_value = "data/unigrams.data",
        value_name = "FILE",
        value_hint = clap::ValueHint::FilePath,
        help = "Data file describing unigrams"
    )]
    pub unigrams_path: PathBuf,

    #[clap(
        long,
        default_value = "data/bigrams.data",
        value_name = "FILE",
        value_hint = clap::ValueHint::FilePath,
        help = "Data file describing bigrams"
    )]
    pub bigrams_path: PathBuf,

    #[clap(
        long,
        default_value = "data/trigrams.data",
        value_name = "FILE",
        value_hint = clap::ValueHint::FilePath,
        help = "Data file describing trigrams"
    )]
    pub trigrams_path: PathBuf,

    #[clap(
        short,
        long,
        value_name = "FILE",
        value_hint = clap::ValueHint::FilePath,
        help = "Evaluate a given layout"
    )]
    pub eval_layout: Option<PathBuf>,
}

pub fn eval_layout<P: AsRef<Path>>(p: P) -> Result<()> {
    let args = Args::parse();
    let eval = LayoutEval::from_args(&args)?;
    let l = load_seeds(p)?;
    let fitness = eval.fitness(&l[0], &())?;
    println!("layout:\n{}", eval.model.format(&l[0].0));
    println!("fitness: {fitness}");
    Ok(())
}

pub fn layout_evolver(cfg: EvolveCfg) -> Result<Evolver<impl Evaluator<Data = ()>>> {
    let args = Args::parse();
    let model = load_model(&args.model_path)?;
    let eval = CachedEvaluator::new(LayoutEval::from_args(&args)?, 1000);
    let genfn = move || {
        let mut keys = model.without_fixed(&model.universe);
        keys.shuffle(&mut rand::rng());
        KeyState(model.with_fixed(&keys))
    };
    if let Some(seed) = args.seed_path {
        let initial_keys = load_seeds(seed)?;
        Ok(Evolver::from_initial(eval, cfg, initial_keys, genfn))
    } else {
        Ok(Evolver::new(eval, cfg, genfn))
    }
}

pub fn evolve(cfg: EvolveCfg) -> Result<()> {
    let evolver = layout_evolver(cfg)?;

    let mut trainer = Trainer::new(
        TrainerCfg::new("memelay")
            .set_termination(Termination::FixedGenerations(20000))
            .set_print_gen(50)
            .set_print_summary(50),
    );
    let _ = trainer.train(evolver, &EmptyDataSampler {})?;

    Ok(())
}

pub fn run() -> Result<()> {
    let args = Args::parse();
    // Remember to update these values if add more mutation/crossover strategies.
    let cfg = EvolveCfg::new(1000)
        .set_mutation(Mutation::Adaptive)
        .set_crossover(Crossover::Adaptive)
        .set_survival(Survival::SpeciesTopProportion(0.1))
        .set_species(Species::TargetNumber(100))
        .set_niching(Niching::None)
        .set_stagnation(Stagnation::ContinuousAfter(200))
        .set_replacement(Replacement::ReplaceChildren(0.5))
        .set_duplicates(Duplicates::DisallowDuplicates)
        .set_par_fitness(true)
        .set_par_dist(true);

    if let Some(p) = args.eval_layout {
        eval_layout(p)?;
    } else {
        evolve(cfg)?;
    }

    Ok(())
}
