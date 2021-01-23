use crate::measurement::F64Measurement;
use criterion::measurement::Measurement;
use criterion::{BenchmarkGroup, Criterion};
use ga::cfg::Cfg;
use ga::distributions::PrintableAscii;
use ga::generation::{Generation, SelectionMethod};
use ga::runner::{RunResult, Runner, Stats};
use ga::util::{count_different, crossover_kpx_rand, mutate_iter};
use ga::Evaluator;
use rand::Rng;
use smallvec::{smallvec, SmallVec};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

mod measurement;

type State = String;

#[derive(Debug, Clone)]
struct BenchEval {
    target: String,
}

impl BenchEval {
    fn new(target: &str) -> Self {
        Self { target: target.to_string() }
    }
}

impl Evaluator for BenchEval {
    type State = State;
    type Fitness = f64;

    fn crossover(&self, _: &Cfg, s1: &State, s2: &State) -> SmallVec<[State; 2]> {
        let mut r = rand::thread_rng();
        let (c1, c2) = crossover_kpx_rand(s1.chars(), s2.chars(), 2, &mut r);
        smallvec![c1, c2]
    }

    fn mutate(&self, cfg: &Cfg, s: &mut State) {
        let mut r = rand::thread_rng();
        *s = mutate_iter(s.chars(), cfg.mutation_rate, |r| r.sample(PrintableAscii), &mut r);
    }

    fn fitness(&self, _: &Cfg, s: &State) -> f64 {
        (self.target.len() - count_different(s.chars(), self.target.chars())) as f64 + 1.0
    }

    fn distance(&self, _: &Cfg, s1: &State, s2: &State) -> f64 {
        count_different(s1.chars(), s2.chars()) as f64
    }
}

#[inline]
fn evolve(target: &str, cfg: Cfg) -> RunResult<BenchEval> {
    const RUNS: usize = 100;
    let mut r = rand::thread_rng();
    let initial = (0..target.len()).map(|_| r.sample::<char, _>(PrintableAscii)).collect();
    let gen = Generation::from_states(vec![initial]);
    let mut runner = Runner::new(BenchEval::new(target), cfg, gen);
    for _ in 0..RUNS {
        runner.run_iter(false);
    }
    runner.run_iter(true)
}

type MetricFn = Box<dyn Fn(Stats<BenchEval>) -> f64>;

fn bench_evolve<M: 'static + Measurement>(
    base_cfg: Cfg,
    g: &mut BenchmarkGroup<'_, M>,
    value: Rc<RefCell<f64>>,
    f: &dyn Fn(Stats<BenchEval>) -> f64,
) {
    let cfgs = [("100 pop", base_cfg)];
    // let cfgs =
    //     cfgs.iter().map(|&(name, cfg)| (name, cfg.with_crossover_rate(0.1))).collect::<Vec<_>>();
    for (name, cfg) in cfgs.iter() {
        g.bench_with_input(*name, cfg, |b, &cfg| {
            b.iter(|| {
                let r = evolve("hello world!", cfg);
                *value.borrow_mut() += f(r.stats.unwrap());
            })
        });
    }
}

fn ga() {
    let value = Rc::new(RefCell::new(0.0));
    let mut c = Criterion::default()
        .configure_from_args()
        .warm_up_time(Duration::new(0, 1)) // Don't need warm-up time for non-time measurement.
        .with_measurement(F64Measurement::new(Rc::clone(&value)));
    let metrics: &[(&'static str, MetricFn)] = &[
        ("best fitness", Box::new(|r| r.best_fitness)),
        ("mean fitness", Box::new(|r| r.mean_fitness)),
        ("dupes", Box::new(|r| r.num_dup as f64)),
        ("mean dist", Box::new(|r| r.mean_distance)),
    ];

    let base_cfg = Cfg::new(100).with_selection_method(SelectionMethod::StochasticUniformSampling);
    for (metric, f) in metrics.iter() {
        let mut g = c.benchmark_group(format!("ga {}", metric));
        bench_evolve(base_cfg, &mut g, Rc::clone(&value), f);
        g.finish();
    }

    let mut c = Criterion::default().configure_from_args();
    let mut g = c.benchmark_group("ga time");
    bench_evolve(base_cfg, &mut g, value, &|_| 0.0);
    g.finish();
}

fn main() {
    ga();
    Criterion::default().configure_from_args().final_summary();
}
