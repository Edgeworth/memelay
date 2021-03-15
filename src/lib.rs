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

use crate::constants::Constants;
use crate::ingest::load_layout;
use crate::layout_eval::LayoutEval;
use crate::models::layout::Layout;
use eyre::Result;
use ga::cfg::{Cfg, Crossover, Mutation, Niching, Species, Survival};
use ga::gen::unevaluated::UnevaluatedGen;
use ga::runner::{Runner, Stats};
use ga::Evaluator;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

pub mod constants;
pub mod ingest;
pub mod layout_eval;
pub mod models;
pub mod path;
pub mod types;

#[derive(Debug, StructOpt)]
#[structopt(name = "hodlr", about = "Hodlr CLI")]
pub struct Args {
    #[structopt(
        long,
        default_value = "data/subset.cfg",
        parse(from_os_str),
        help = "Config file describing target layout and costs"
    )]
    pub cfg_path: PathBuf,

    #[structopt(
        long,
        default_value = "data/layer0.data",
        parse(from_os_str),
        help = "Corpus file describing typing data to optimise to"
    )]
    pub corpus_path: PathBuf,

    #[structopt(short, long, parse(from_os_str), help = "Evaluate a given layout")]
    pub eval_layout: Option<PathBuf>,

    #[structopt(flatten)]
    pub cnst: Constants,
}

pub fn eval_layout<P: AsRef<Path>>(eval: LayoutEval, p: P) -> Result<()> {
    let l = load_layout(p)?;
    let fitness = eval.fitness(&l);
    println!("layout: {}", eval.layout_cfg.format(&l));
    println!("fitness: {}", fitness);
    Ok(())
}

pub fn evolve(eval: LayoutEval, cfg: Cfg) -> Result<()> {
    let initial = load_layout("data/layer0.layout")?;
    let initial = (0..cfg.pop_size).map(|_| initial.clone()).collect();
    let initial = UnevaluatedGen::initial::<LayoutEval>(initial, &cfg);
    let mut runner = Runner::new(eval.clone(), cfg, initial);

    for i in 0..eval.cnst.runs {
        let mut r = runner.run_iter()?;
        println!("Generation {}: {}", i + 1, r.gen.best().base_fitness);
        if i % 10 == 0 {
            println!("Stats: {:?}", Stats::from_run(&mut r, &runner));
            println!("{}", eval.layout_cfg.format(&r.gen.best().state.0));
        }
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let args = Args::from_args();
    let eval = LayoutEval::from_args(&args)?;
    // Remember to update these values if add more mutation/crossover strategies.
    let cfg = Cfg::new(eval.cnst.pop_size)
        .with_mutation(Mutation::Fixed(vec![0.001, 0.7]))
        .with_crossover(Crossover::Fixed(vec![0.0]))
        .with_survival(Survival::TopProportion(0.2))
        .with_species(Species::None)
        .with_niching(Niching::None)
        .with_par_fitness(true);

    if let Some(p) = args.eval_layout {
        eval_layout(eval, p)?;
    } else {
        evolve(eval, cfg)?;
    }

    Ok(())
}
