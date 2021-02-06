use eyre::{eyre, Result};
use ga::cfg::{Cfg, Crossover, Mutation, Selection};
use ga::examples::griewank::griewank_runner;
use ga::examples::knapsack::knapsack_runner;
use ga::examples::rastrigin::rastrigin_runner;
use ga::examples::{all_cfg, none_cfg};
use ga::runner::Runner;
use ga::Evaluator;
use grapher::Grapher;

type RunnerFn<E> = dyn Fn(&Cfg) -> Runner<E>;

fn eval_run<E: Evaluator>(
    g: &mut Grapher,
    name: &str,
    run_id: &str,
    base_cfg: Cfg,
    runner_fn: &RunnerFn<E>,
) -> Result<()> {
    const SAMPLES: usize = 100;
    for _ in 0..SAMPLES {
        let cfgs = [("100 pop", base_cfg)];
        for (cfg_name, cfg) in cfgs.iter() {
            let mut runner = runner_fn(cfg);
            for _ in 0..100 {
                runner.run_iter(false)?;
            }
            let r = runner.run_iter(true)?.stats.unwrap();

            g.add(&format!("{}:{}:best fitness", name, cfg_name), run_id, r.best_fitness);
            g.add(&format!("{}:{}:mean fitness", name, cfg_name), run_id, r.mean_fitness);
            g.add(&format!("{}:{}:dupes", name, cfg_name), run_id, r.num_dup as f64);
            g.add(&format!("{}:{}:mean dist", name, cfg_name), run_id, r.mean_distance);
            g.add(&format!("{}:{}:species", name, cfg_name), run_id, r.num_species as f64);
        }
    }
    Ok(())
}

fn run_grapher<E: Evaluator>(name: &str, base_cfg: Cfg, runner_fn: &RunnerFn<E>) -> Result<()> {
    let mut g = Grapher::new();
    let mod_cfg = all_cfg();
    eval_run(&mut g, name, "def", base_cfg, runner_fn)?;
    eval_run(&mut g, name, "mod", mod_cfg, runner_fn)?;
    g.analyse();
    Ok(())
}

fn run_once<E: Evaluator>(cfg: Cfg, runner_fn: &RunnerFn<E>) -> Result<()> {
    let mut runner = runner_fn(&cfg);
    for i in 0..100 {
        let detail = i % 10 == 0;
        let r = runner.run_iter(detail)?;
        println!("Generation {}: {}", i + 1, r.gen.best().base_fitness);
        if detail {
            println!("  {:?}\n  best: {:?}", r.stats.unwrap(), r.gen.best().state);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cfg = none_cfg();
    // let cfg = all_cfg();
    // run_grapher("knapsack", cfg, &knapsack_runner)?;
    // run_grapher("rastrigin", cfg, &|cfg| rastrigin_runner(2, cfg))?;
    run_grapher("griewank", cfg, &|cfg| griewank_runner(2, cfg))?;
    // run_once(cfg, &|cfg| rastrigin_runner(2, cfg))?;
    Ok(())
}
