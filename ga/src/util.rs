use num_traits::{NumCast, ToPrimitive};
use rand::prelude::IteratorRandom;
use rand::Rng;
use smallvec::SmallVec;
use std::borrow::Borrow;
use std::iter::{FromIterator, Iterator};

// Random point K-point crossover.
pub fn crossover_kpx_rand<O: FromIterator<T::Item>, T: IntoIterator, R: Rng + ?Sized>(
    s1: T,
    s2: T,
    k: usize,
    r: &mut R,
) -> (O, O) {
    let s1: Vec<_> = s1.into_iter().collect();
    let s2: Vec<_> = s2.into_iter().collect();
    let xpoints = (0..s1.len()).choose_multiple(r, k);
    crossover_kpx(s1, s2, &xpoints)
}

// K-point crossover.
pub fn crossover_kpx<O: FromIterator<T::Item>, T: IntoIterator>(
    s1: T,
    s2: T,
    xpoints: &[usize],
) -> (O, O) {
    let mut s1 = s1.into_iter();
    let mut s2 = s2.into_iter();
    let mut o1 = Vec::new();
    let mut o2 = Vec::new();
    let mut xpoints: SmallVec<[usize; 4]> = SmallVec::from_slice(xpoints);
    xpoints.push(0);
    xpoints.sort_unstable();
    for [prev, cur] in xpoints.array_windows::<2>() {
        o1.extend(s1.by_ref().take(cur - prev));
        o2.extend(s2.by_ref().take(cur - prev));
        std::mem::swap(&mut s1, &mut s2);
    }
    o1.extend(s1);
    o2.extend(s2);
    (o1.into_iter().collect(), o2.into_iter().collect())
}

// Uniform crossover.
pub fn crossover_ux<O: FromIterator<T::Item>, T: IntoIterator, R: Rng + ?Sized>(
    s1: T,
    s2: T,
    r: &mut R,
) -> (O, O) {
    let s1 = s1.into_iter();
    let s2 = s2.into_iter();
    let mut o1 = Vec::new();
    let mut o2 = Vec::new();
    for (v1, v2) in s1.zip(s2) {
        if r.gen::<bool>() {
            o1.push(v1);
            o2.push(v2);
        } else {
            o1.push(v2);
            o2.push(v1);
        }
    }
    (o1.into_iter().collect(), o2.into_iter().collect())
}

// Mutation:
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

// Calculating fitnesses:
pub fn count_different<V: PartialEq, A: IntoIterator<Item = V>, B: IntoIterator<Item = V>>(
    s1: A,
    s2: B,
) -> usize {
    let mut s1 = s1.into_iter();
    let mut s2 = s2.into_iter();
    let mut count = 0;
    loop {
        let v1 = s1.next();
        let v2 = s2.next();
        if v1.is_none() && v2.is_none() {
            return count;
        }
        if v1 != v2 {
            count += 1;
        }
    }
}

// Stochastic universal sampling
pub fn sus<'a, V: 'a + Copy + ToPrimitive, T: IntoIterator<Item = &'a V>, R: Rng + ?Sized>(
    w: T,
    k: usize,
    r: &mut R,
) -> Vec<usize> {
    let w = w.into_iter().map(|v| NumCast::from(*v.borrow()).unwrap()).collect::<Vec<f64>>();
    let sum = w.iter().fold(0.0, |a, b| a + b);
    if k == 0 || sum == 0.0 {
        return vec![];
    }
    let step = sum / k as f64;
    let mut idxs = Vec::new();
    let mut idx = 0;
    let mut cursum = 0.0;
    let mut cursor = r.gen_range(0.0..step);
    for _ in 0..k {
        while cursum + w[idx] < cursor {
            cursum += w[idx];
            idx += 1;
        }
        idxs.push(idx);
        cursor += step;
    }
    idxs
}

// Combining fitnesses:
pub fn combine_fitness(cur_fitness: u128, next: u128, max_next: u128) -> u128 {
    let mut unit = 1;
    while unit < max_next {
        unit *= 10;
    }
    cur_fitness * unit * 10 + next
}

pub fn combine_cost(cur_fitness: u128, next: u128, max_next: u128) -> u128 {
    combine_fitness(cur_fitness, max_next - next, max_next)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_crossover_kpx() {
        assert_eq!(
            crossover_kpx("abcd".chars(), "wxyz".chars(), &[3]),
            ("abcz".to_string(), "wxyd".to_string())
        );
        assert_eq!(
            crossover_kpx("abcd".chars(), "wxyz".chars(), &[1, 2]),
            ("axcd".to_string(), "wbyz".to_string())
        );
    }

    #[test]
    fn test_crossover_ux() {
        let mut r = StepRng::new(1 << 31, 1 << 31);
        assert_eq!(
            crossover_ux("abcd".chars(), "wxyz".chars(), &mut r),
            ("axcz".to_string(), "wbyd".to_string())
        );
    }

    #[test]
    fn test_count_different() {
        assert_eq!(count_different(&[1], &[1]), 0);
        assert_eq!(count_different(&[1], &[2]), 1);
        assert_eq!(count_different(&[1], &[1, 2]), 1);
        assert_eq!(count_different(&[1, 2], &[1]), 1);
    }

    #[test]
    fn test_sus() {
        let mut r = StepRng::new(1 << 31, 1 << 31);
        assert_eq!(sus::<u32, _, _>(&[], 0, &mut r), []);
        assert_eq!(sus::<u32, _, _>(&[], 1, &mut r), []);
        assert_eq!(sus(&[1], 0, &mut r), []);
        assert_eq!(sus(&[1], 1, &mut r), [0]);
        assert_eq!(sus(&[1], 1, &mut r), [0]);
        assert_eq!(sus(&[1, 1], 1, &mut r), [0]);
        assert_eq!(sus(&[1, 1], 2, &mut r), [0, 1]);
        assert_eq!(sus(&[1, 2], 3, &mut r), [0, 1, 1]);
    }
}
