use ga::cfg::Cfg;
use ga::examples::knapsack::knapsack_runner;
use ga::examples::none_cfg;
use ga::runner::Runner;
use ga::Evaluator;
use grapher::Grapher;

fn run_grapher<E: Evaluator>(name: &str, base_cfg: Cfg, runner_fn: &dyn Fn(&Cfg) -> Runner<E>) {
    const SAMPLES: usize = 2;
    let mut g = Grapher::new();
    for _ in 0..SAMPLES {
        let cfgs = [("100 pop", base_cfg)];
        for (cfg_name, cfg) in cfgs.iter() {
            let mut runner = runner_fn(cfg);
            const RUNS: usize = 100;
            for _ in 0..RUNS {
                runner.run_iter(false);
            }
            let r = runner.run_iter(true).stats.unwrap();

            g.add(&format!("{}:{}:best fitness", name, cfg_name), "cur", r.best_fitness);
            g.add(&format!("{}:{}:mean fitness", name, cfg_name), "cur", r.mean_fitness);
            g.add(&format!("{}:{}:dupes", name, cfg_name), "cur", r.num_dup as f64);
            g.add(&format!("{}:{}:mean dist", name, cfg_name), "cur", r.mean_distance);
            g.add(&format!("{}:{}:species", name, cfg_name), "cur", r.num_species as f64);
        }
    }
}

fn main() {
    let cfg = none_cfg();
    // let cfg = all_cfg();
    run_grapher("knapsack", cfg, &knapsack_runner);
}
