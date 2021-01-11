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

use crate::constants::Constants;
use crate::ga::runner::{Generation, Runner};
use crate::ga::{Cfg, Evaluator};
use crate::ingest::load_layout;
use crate::layout_eval::LayoutEval;
use crate::models::layout::Layout;
use crate::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod constants;
mod ga;
mod ingest;
mod layout_eval;
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

    #[structopt(short, long, parse(from_os_str), help = "Evaluate a given layout")]
    eval_layout: Option<PathBuf>,

    #[structopt(flatten)]
    cnst: Constants,
}

pub fn eval_layout<P: AsRef<Path>>(eval: LayoutEval, cfg: Cfg, p: P) -> Result<()> {
    let l = load_layout(p)?;
    let fitness = eval.fitness(&cfg, &l);
    println!("layout: {}", eval.layout_cfg.format(&l));
    println!("fitness: {}", fitness);
    Ok(())
}

pub fn evolve(eval: LayoutEval, cfg: Cfg) -> Result<()> {
    let initial = (0..cfg.pop_size)
        .map(|_| Layout::rand_with_size(eval.num_physical(), 2, &eval.cnst))
        .collect();
    let mut runner = Runner::new(eval.clone(), cfg, Generation::from_states(initial));

    let mut best;
    for i in 0..eval.cnst.runs {
        best = runner.run_iter();
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
    let cfg = Cfg { xover_rate: 0.3, pop_size: eval.cnst.pop_size, top_prop: 0.1 };

    if let Some(p) = args.eval_layout {
        eval_layout(eval, cfg, p)?;
    } else {
        evolve(eval, cfg)?;
    }

    Ok(())
}
