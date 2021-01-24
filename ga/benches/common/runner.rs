use crate::common::measurement::F64Measurement;
use criterion::measurement::Measurement;
use criterion::{BenchmarkGroup, Criterion};
use ga::cfg::Cfg;
use ga::runner::{Runner, Stats};
use ga::Evaluator;
use num_traits::NumCast;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

type MetricFn<E> = Box<dyn Fn(Stats<E>) -> f64>;

fn bench_evolve<E: Evaluator, M: 'static + Measurement>(
    base_cfg: Cfg,
    g: &mut BenchmarkGroup<'_, M>,
    value: Rc<RefCell<f64>>,
    runner_fn: &dyn Fn(&Cfg) -> Runner<E>,
    stats_fn: &dyn Fn(Stats<E>) -> f64,
) {
    const RUNS: usize = 100;
    let cfgs = [("100 pop", base_cfg)];
    // let cfgs =
    //     cfgs.iter().map(|&(name, cfg)| (name, cfg.with_crossover_rate(0.1))).collect::<Vec<_>>();
    for (name, cfg) in cfgs.iter() {
        g.bench_with_input(*name, cfg, |b, cfg| {
            b.iter(|| {
                let mut runner = runner_fn(cfg);
                for _ in 0..RUNS {
                    runner.run_iter(false);
                }
                let r = runner.run_iter(true);
                *value.borrow_mut() += stats_fn(r.stats.unwrap());
            })
        });
    }
}

pub fn run<E: Evaluator>(name: &str, base_cfg: Cfg, runner_fn: &dyn Fn(&Cfg) -> Runner<E>) {
    let value = Rc::new(RefCell::new(0.0));
    let mut c = Criterion::default()
        .configure_from_args()
        .sample_size(200)
        .warm_up_time(Duration::new(0, 1)) // Don't need warm-up time for non-time measurement.
        .with_measurement(F64Measurement::new(Rc::clone(&value)));
    let metrics: &[(&'static str, MetricFn<E>)] = &[
        ("best fitness", Box::new(|r| NumCast::from(r.best_fitness).unwrap())),
        ("mean fitness", Box::new(|r| NumCast::from(r.mean_fitness).unwrap())),
        ("dupes", Box::new(|r| r.num_dup as f64)),
        ("mean dist", Box::new(|r| r.mean_distance)),
    ];

    for (metric, stats_fn) in metrics.iter() {
        let mut g = c.benchmark_group(format!("{}:{}", name, metric));
        bench_evolve(base_cfg, &mut g, Rc::clone(&value), runner_fn, stats_fn);
        g.finish();
    }

    let mut c = Criterion::default().configure_from_args();
    let mut g = c.benchmark_group(format!("{} time", name));
    bench_evolve(base_cfg, &mut g, value, runner_fn, &|_| 0.0);
    g.finish();
}
