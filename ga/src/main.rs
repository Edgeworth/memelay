use ga::cfg::{Cfg, Selection};
use ga::examples::knapsack::knapsack_runner;
use ga::examples::none_cfg;
use ga::runner::Runner;
use ga::Evaluator;
use grapher::Grapher;

type RunnerFn<E> = dyn Fn(&Cfg) -> Runner<E>;

fn eval_run<E: Evaluator>(g: &mut Grapher, name: &str, run_id: &str, base_cfg: Cfg, runner_fn: &RunnerFn<E>) {
    const SAMPLES: usize = 2;
    for _ in 0..SAMPLES {
        let cfgs = [("100 pop", base_cfg)];
        for (cfg_name, cfg) in cfgs.iter() {
            let mut runner = runner_fn(cfg);
            const RUNS: usize = 100;
            for _ in 0..RUNS {
                runner.run_iter(false);
            }
            let r = runner.run_iter(true).stats.unwrap();

            g.add(&format!("{}:{}:best fitness", name, cfg_name), run_id, r.best_fitness);
            g.add(&format!("{}:{}:mean fitness", name, cfg_name), run_id, r.mean_fitness);
            g.add(&format!("{}:{}:dupes", name, cfg_name), run_id, r.num_dup as f64);
            g.add(&format!("{}:{}:mean dist", name, cfg_name), run_id, r.mean_distance);
            g.add(&format!("{}:{}:species", name, cfg_name), run_id, r.num_species as f64);
        }
    }
}

fn run_grapher<E: Evaluator>(name: &str, base_cfg: Cfg, runner_fn: &RunnerFn<E>) {
    let mut g = Grapher::new();
    eval_run(&mut g, name, "rws", base_cfg.with_selection(Selection::Roulette), runner_fn);
    eval_run(&mut g, name, "sus", base_cfg, runner_fn);
    g.analyse();
}

fn main() {
    let cfg = none_cfg();
    // let cfg = all_cfg();
    run_grapher("knapsack", cfg, &knapsack_runner);
}
