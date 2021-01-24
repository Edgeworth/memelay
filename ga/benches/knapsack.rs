use criterion::Criterion;
use ga::examples::knapsack::knapsack_runner;
use ga::examples::{none_cfg};

mod common;

fn main() {
    let base_cfg = none_cfg();
    // let base_cfg = all_cfg();
    common::runner::run("knapsack", base_cfg, &knapsack_runner);
    Criterion::default().configure_from_args().final_summary();
}
