#![warn(rust_2018_idioms, clippy::all)]
#![feature(
    const_fn,
    option_result_contains,
    trait_alias,
    iterator_fold_self,
    type_alias_impl_trait,
    partition_point,
    bool_to_option,
    map_first_last,
    option_unwrap_none,
    destructuring_assignment
)]

use crate::constants::Constants;
use crate::ingest::load_layout;
use crate::layout_eval::LayoutEval;
use eyre::Result;
use ga::cfg::Cfg;
use ga::runner::{Generation, Runner};
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
        default_value = "data/moonlander.cfg",
        parse(from_os_str),
        help = "Config file describing target layout and costs"
    )]
    pub cfg_path: PathBuf,

    #[structopt(
        long,
        default_value = "data/bench.data",
        parse(from_os_str),
        help = "Corpus file describing typing data to optimise to"
    )]
    pub corpus_path: PathBuf,

    #[structopt(short, long, parse(from_os_str), help = "Evaluate a given layout")]
    pub eval_layout: Option<PathBuf>,

    #[structopt(flatten)]
    pub cnst: Constants,
}

pub fn eval_layout<P: AsRef<Path>>(eval: LayoutEval, cfg: Cfg, p: P) -> Result<()> {
    let l = load_layout(p)?;
    let fitness = eval.fitness(&cfg, &l);
    println!("layout: {}", eval.layout_cfg.format(&l));
    println!("fitness: {}", fitness);
    Ok(())
}

pub fn evolve(eval: LayoutEval, cfg: Cfg) -> Result<()> {
    // Start from a base with all keys available.
    let initial = load_layout("data/all_keys.layout")?;
    let initial = (0..cfg.pop_size).map(|_| initial.clone()).collect();
    let mut runner = Runner::new(eval.clone(), cfg, Generation::from_states(initial));

    let mut best;
    for i in 0..eval.cnst.runs {
        best = runner.run_iter().gen.best();
        println!("Generation: {} score: {:.3?}", i, best.fitness);
        if i % 10 == 0 {
            println!("{}", eval.layout_cfg.format(&best.state));
        }
    }
    // let fitness = Fitness::new(eval.clone());
    // for (idx, mem) in pop.members_mut().iter().take(10).enumerate() {
    //     let mut l = mem.member.write().unwrap();
    //     let val = fitness.solve(&mut l);
    //     let fmt = eval.layout_cfg.format_solution(&l);
    //     println!("Soln {} fitness {}: {}", idx, val, fmt);
    // }

    Ok(())
}

pub fn run() -> Result<()> {
    let args = Args::from_args();
    let eval = LayoutEval::from_args(&args)?;
    let cfg = Cfg::new(eval.cnst.pop_size);

    if let Some(p) = args.eval_layout {
        eval_layout(eval, cfg, p)?;
    } else {
        evolve(eval, cfg)?;
    }

    Ok(())
}
