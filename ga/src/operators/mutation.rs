use rand::prelude::IteratorRandom;
use rand::Rng;
use std::iter::{FromIterator, Iterator};

// Replaces a random value in |s| with |v|.
pub fn replace_rand<O: FromIterator<T::Item>, T: IntoIterator, R: Rng + ?Sized>(
    s: T,
    v: T::Item,
    r: &mut R,
) -> O {
    let mut o: Vec<_> = s.into_iter().collect();
    if let Some(ov) = o.iter_mut().choose(r) {
        *ov = v;
    }
    o.into_iter().collect()
}

// Mutates with the given rate.
pub fn mutate_iter<O: FromIterator<T::Item>, T: IntoIterator, R: Rng + ?Sized>(
    s: T,
    rate: f64,
    f: impl Fn(&mut R) -> T::Item,
    r: &mut R,
) -> O {
    s.into_iter().map(|v| if r.gen::<f64>() < rate { f(r) } else { v }).collect()
}
