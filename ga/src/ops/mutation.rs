use rand::prelude::IteratorRandom;
use rand::Rng;
use rand_distr::StandardNormal;

// Replaces a random value in |s| with |v|.
pub fn replace_rand<T, R: Rng + ?Sized>(s: &mut [T], v: T, r: &mut R) {
    if let Some(ov) = s.iter_mut().choose(r) {
        *ov = v;
    }
}

// Mutates with the given rate.
pub fn mutate_rate<T, R: Rng + ?Sized>(s: &mut [T], rate: f64, f: impl Fn(&mut R) -> T, r: &mut R) {
    for v in s {
        if r.gen::<f64>() < rate {
            *v = f(r);
        }
    }
}

pub fn mutate_lognorm<R: Rng + ?Sized>(f: f64, std: f64, r: &mut R) -> f64 {
    f * std::f64::consts::E.powf(std * r.sample::<f64, _>(StandardNormal))
}
