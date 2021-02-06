use crate::cfg::Cfg;
use crate::examples::func::{func_runner, FuncEvaluator, FuncState};
use crate::runner::Runner;
use crate::FitnessFn;
use std::f64::consts::{E, PI};

pub fn ackley_runner(dim: usize, cfg: &Cfg) -> Runner<FuncEvaluator<impl FitnessFn<FuncState>>> {
    func_runner(
        dim,
        -32.768,
        32.768,
        |s: &FuncState| {
            const a: f64 = 20.0;
            const b: f64 = 0.2;
            const c: f64 = 2.0 * PI;
            let d = s.len() as f64;
            let mut squares = 0.0;
            let mut cos = 0.0;
            for &x in s.iter() {
                squares += x * x;
                cos += (c * x).cos();
            }
            let squares = -b * (squares / d).sqrt();
            let cos = cos / d;
            let v = -a * squares.exp() - cos.exp() + a + E;
            // Convert to a maximisation problem
            1.0 / (1.0 + v)
        },
        cfg,
    )
}
