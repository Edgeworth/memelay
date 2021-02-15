use eyre::Result;
use ga::cfg::Cfg;
use ga::examples::ackley::ackley_runner;
use ga::examples::griewank::griewank_runner;
use ga::examples::knapsack::knapsack_runner;
use ga::examples::rastrigin::rastrigin_runner;
use ga::examples::target_string::target_string_runner;
use ga::examples::{all_cfg, none_cfg};
use ga::hyper::hyper_all;
use ga::runner::{Runner, RunnerFn};
use ga::Evaluator;
use grapher::Grapher;

fn eval_run<E: Evaluator>(
    g: &mut Grapher,
    name: &str,
    run_id: &str,
    base_cfg: Cfg,
    runner_fn: &impl RunnerFn<E>,
) -> Result<()> {
    const SAMPLES: usize = 100;
    let cfgs = [("100 pop", base_cfg)];
    for _ in 0..SAMPLES {
        for (cfg_name, cfg) in cfgs.iter() {
            let mut runner = runner_fn(cfg.clone());
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

fn run_grapher<E: Evaluator>(
    name: &str,
    base_cfg: Cfg,
    runner_fn: &impl RunnerFn<E>,
) -> Result<()> {
    let mut g = Grapher::new();
    let mod_cfg = all_cfg();
    eval_run(&mut g, name, "def", base_cfg, runner_fn)?;
    eval_run(&mut g, name, "mod", mod_cfg, runner_fn)?;
    g.analyse();
    Ok(())
}

fn run_once<E: Evaluator>(mut runner: Runner<E>) -> Result<()> {
    for i in 0..1000 {
        let detail = true; // i % 10 == 0;
        let r = runner.run_iter(detail)?;
        println!("Generation {}: {}", i + 1, r.gen.best().base_fitness);
        if detail {
            println!("  {:?}\n  best: {:?}", r.stats.unwrap(), r.gen.best().state);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    // let cfg = none_cfg();
    // let cfg = all_cfg();
    // run_grapher("knapsack", cfg.clone(), &knapsack_runner)?;
    // run_grapher("rastrigin", cfg.clone(), &|cfg| rastrigin_runner(2, cfg))?;
    // run_grapher("griewank", cfg.clone(), &|cfg| griewank_runner(2, cfg))?;
    // run_grapher("ackley", cfg.clone(), &|cfg| ackley_runner(2, cfg))?;
    // run_grapher("string", cfg, &target_string_runner)?;
    // run_once(rastrigin_runner(2, all_cfg()))?;
    run_once(hyper_all())?;
    // run_once(hyper_runner(&knapsack_runner))?;
    // run_once(hyper_runner(&target_string_runner))?;
    // run_once(hyper_runner))?;
    Ok(())
}
