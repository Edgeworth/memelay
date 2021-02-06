use rand::prelude::IteratorRandom;
use rand::Rng;
use rand_distr::StandardNormal;
use std::f64::consts::E;

// Discrete mutation operators:
// Replaces a random value in |s| with |v|.
pub fn replace_rand<T, R: Rng + ?Sized>(s: &mut [T], v: T, r: &mut R) {
    if let Some(ov) = s.iter_mut().choose(r) {
        *ov = v;
    }
}

// Mutates with the given rate.
pub fn mutate_rate<T: Copy, R: Rng + ?Sized>(
    s: &mut [T],
    rate: f64,
    f: impl Fn(T, &mut R) -> T,
    r: &mut R,
) {
    for v in s {
        if r.gen::<f64>() < rate {
            *v = f(*v, r);
        }
    }
}

// Real mutation operators:
// Random value taken from the uniform distribution on |range|.
pub fn mutate_uniform<R: Rng + ?Sized>(st: f64, en: f64, r: &mut R) -> f64 {
    r.gen_range(st..=en)
}

// Mutate |v| by a value from N(0, std). It's usual to use the mutation rate as |std|.
// May want to clamp the value to a range afterwards.
pub fn mutate_normal<R: Rng + ?Sized>(v: f64, std: f64, r: &mut R) -> f64 {
    v + std * r.sample::<f64, _>(StandardNormal)
}

// Mutate s.t. v' = v * e^(std * N(0, 1)).
// May want to clamp the value to a range afterwards.
pub fn mutate_lognorm<R: Rng + ?Sized>(v: f64, std: f64, r: &mut R) -> f64 {
    v * E.powf(std * r.sample::<f64, _>(StandardNormal))
}
