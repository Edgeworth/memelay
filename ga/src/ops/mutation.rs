use num_traits::Num;
use rand::prelude::IteratorRandom;
use rand::Rng;
use rand_distr::uniform::SampleUniform;
use rand_distr::StandardNormal;
use std::f64::consts::E;

// Discrete mutation operators:
// Replaces a random value in |s| with |v|.
pub fn mutate_reset<T>(s: &mut [T], v: T) {
    let mut r = rand::thread_rng();
    if let Some(ov) = s.iter_mut().choose(&mut r) {
        *ov = v;
    }
}

// Mutates with the given rate.
pub fn mutate_rate<T: Copy>(s: &mut [T], rate: f64, mut f: impl FnMut(T) -> T) {
    let mut r = rand::thread_rng();
    for v in s {
        if r.gen::<f64>() < rate {
            *v = f(*v);
        }
    }
}

// Real mutation operators:
// Random value taken from the uniform distribution on |range|.
pub fn mutate_uniform(st: f64, en: f64) -> f64 {
    let mut r = rand::thread_rng();
    r.gen_range(st..=en)
}

// Mutate |v| by a value from N(0, std). It's usual to use the mutation rate as |std|.
// May want to clamp the value to a range afterwards.
pub fn mutate_normal(v: f64, std: f64) -> f64 {
    let mut r = rand::thread_rng();
    v + std * r.sample::<f64, _>(StandardNormal)
}

// Mutate s.t. v' = v * e^(std * N(0, 1)).
// May want to clamp the value to a range afterwards.
pub fn mutate_lognorm(v: f64, std: f64) -> f64 {
    let mut r = rand::thread_rng();
    v * E.powf(std * r.sample::<f64, _>(StandardNormal))
}

// Number mutation operators:
pub fn mutate_creep<T: Num + SampleUniform + PartialOrd>(v: T, max_diff: T) -> T {
    let mut r = rand::thread_rng();
    let diff = r.gen_range(T::zero()..max_diff);
    if r.gen::<bool>() { v - diff } else { v + diff }
}
