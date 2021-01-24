use criterion::Criterion;
use ga::examples::target_string::target_string_runner;
use ga::examples::{none_cfg};

mod common;

fn main() {
    let base_cfg = none_cfg();
    // let base_cfg = all_cfg();
    common::runner::run("target_string", base_cfg, &target_string_runner);
    Criterion::default().configure_from_args().final_summary();
}
