use rand::prelude::IteratorRandom;
use rand::Rng;

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
