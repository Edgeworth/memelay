use ga::examples::knapsack::knapsack_runner;
use ga::examples::{none_cfg};

fn main() {
    const RUNS: usize = 100;
    let cfg = none_cfg();
    // let cfg = all_cfg();
    let mut runner = knapsack_runner(&cfg);
    for i in 0..RUNS {
        let r = runner.run_iter(true);
        println!("Generation {}: {}", i + 1, r.gen);
        println!("{:?}\n", r.stats.unwrap());
    }
}
