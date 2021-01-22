use crate::measurement::F64Measurement;
use criterion::measurement::Measurement;
use criterion::{BenchmarkGroup, Criterion};
use ga::cfg::Cfg;
use ga::distributions::PrintableAscii;
use ga::runner::SelectionMethod::{RouletteWheel, StochasticUniformSampling};
use ga::runner::{Generation, Runner};
use ga::util::{count_different, crossover_kpx_rand, replace_rand};
use ga::Evaluator;
use rand::Rng;
use smallvec::{smallvec, SmallVec};
use std::cell::RefCell;
use std::rc::Rc;

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

    fn mutate(&self, _: &Cfg, s: &mut State) {
        let mut r = rand::thread_rng();
        *s = replace_rand(s.chars(), r.sample(PrintableAscii), &mut r);
    }

    fn fitness(&self, _: &Cfg, s: &State) -> f64 {
        (self.target.len() - count_different(s.chars(), self.target.chars())) as f64 + 1.0
    }

    fn distance(&self, _: &Cfg, _s1: &State, _s3: &State) -> f64 {
        todo!()
    }
}

struct EvolveResult {
    num_runs: usize,
    last_gen: Generation<BenchEval>,
}

#[inline]
fn evolve(target: &str, cfg: Cfg) -> EvolveResult {
    let mut r = rand::thread_rng();
    let initial = (0..target.len()).map(|_| r.sample::<char, _>(PrintableAscii)).collect();
    let gen = Generation::from_states(vec![initial]);
    let mut runner = Runner::new(BenchEval::new(target), cfg, gen);
    let mut num_runs = 0;
    loop {
        num_runs += 1;
        let g = runner.run_iter().gen;
        // println!(
        //     "Gen #{} mean: {:.3?}, pop: {}, best: {:.1?}, val: {}",
        //     num_runs,
        //     g.mean_fitness(),
        //     g.individuals().len(),
        //     g.best().fitness,
        //     g.best().state
        // );
        if g.best().state == target {
            return EvolveResult { num_runs, last_gen: g };
        }
    }
}

type MetricFn = Box<dyn Fn(EvolveResult) -> f64>;

fn bench_evolve<M: 'static + Measurement>(
    g: &mut BenchmarkGroup<'_, M>,
    value: Rc<RefCell<f64>>,
    f: &dyn Fn(EvolveResult) -> f64,
) {
    const POP: usize = 100;
    for (name, cfg) in &[
        ("sus", Cfg::new(POP).with_selection_method(StochasticUniformSampling)),
        ("rws", Cfg::new(POP).with_selection_method(RouletteWheel)),
    ] {
        g.bench_with_input(*name, cfg, |b, &cfg| {
            b.iter(|| {
                let r = evolve("hello world!", cfg);
                *value.borrow_mut() += f(r);
            })
        });
    }
}

fn ga() {
    let value = Rc::new(RefCell::new(0.0));
    let mut c = Criterion::default()
        .configure_from_args()
        .with_measurement(F64Measurement::new(Rc::clone(&value)));
    let metrics: &[(&'static str, MetricFn)] = &[
        ("num runs", Box::new(|r| r.num_runs as f64)),
        ("mean fitness", Box::new(|r| r.last_gen.mean_fitness().unwrap())),
    ];
    for (metric, f) in metrics.iter() {
        let mut g = c.benchmark_group(format!("ga {}", metric));
        bench_evolve(&mut g, Rc::clone(&value), f);
        g.finish();
    }

    let mut c = Criterion::default().configure_from_args();
    let mut g = c.benchmark_group("ga time");
    bench_evolve(&mut g, value, &|_| 0.0);
    g.finish();
}

fn main() {
    ga();
    Criterion::default().configure_from_args().final_summary();
}
