use rand::prelude::IteratorRandom;
use rand::Rng;
use smallvec::SmallVec;

// Discrete crossover operators:
// Random point K-point crossover.
pub fn crossover_kpx_rand<T, R: Rng + ?Sized>(s1: &mut [T], s2: &mut [T], k: usize, r: &mut R) {
    let xpoints = (0..s1.len()).choose_multiple(r, k);
    crossover_kpx(s1, s2, &xpoints)
}

// K-point crossover.
pub fn crossover_kpx<T>(s1: &mut [T], s2: &mut [T], xpoints: &[usize]) {
    let mut xpoints: SmallVec<[usize; 4]> = SmallVec::from_slice(xpoints);
    let min = s1.len().min(s2.len());
    xpoints.push(min);
    xpoints.sort_unstable();
    for &[st, en] in xpoints.array_chunks::<2>() {
        for i in st..en {
            std::mem::swap(&mut s1[i], &mut s2[i]);
        }
    }
}

// Uniform crossover.
pub fn crossover_ux<T, R: Rng + ?Sized>(s1: &mut [T], s2: &mut [T], r: &mut R) {
    let min = s1.len().min(s2.len());
    for i in 0..min {
        if r.gen::<bool>() {
            std::mem::swap(&mut s1[i], &mut s2[i]);
        }
    }
}

// Real crossover operators:

// Whole arithemtic recombination:
pub fn crossover_arith(s1: &mut [f64], s2: &mut [f64], alpha: f64) {
    let min = s1.len().min(s2.len());
    for i in 0..min {
        let c1 = alpha * s1[i] + (1.0 - alpha) * s2[i];
        let c2 = alpha * s2[i] + (1.0 - alpha) * s1[i];
        (s1[i], s2[i]) = (c1, c2);
    }
}

pub fn crossover_arith_rand<R: Rng + ?Sized>(s1: &mut [f64], s2: &mut [f64], r: &mut R) {
    crossover_arith(s1, s2, r.gen())
}

// Blend crossover. For each element x < y, randomly generate a value in
// [x - |y - x| * alpha, y + |y - x| * alpha]. A good choice for alpha is 0.5.
pub fn crossover_blx<R: Rng + ?Sized>(s1: &mut [f64], s2: &mut [f64], alpha: f64, r: &mut R) {
    let min = s1.len().min(s2.len());
    for i in 0..min {
        let x = s1[i].min(s2[i]);
        let y = s1[i].max(s2[i]);
        let dist = y - x;
        let left = x - dist * alpha;
        let right = y + dist * alpha;
        s1[i] = r.gen_range(left..right);
        s2[i] = r.gen_range(left..right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ops::initial::{str_to_vec, vec_to_str};
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_crossover_1px() {
        let mut a = str_to_vec("abcd");
        let mut b = str_to_vec("wxyz");
        crossover_kpx(&mut a, &mut b, &[3]);
        assert_eq!(vec_to_str(&a), "abcz");
        assert_eq!(vec_to_str(&b), "wxyd");
    }

    #[test]
    fn test_crossover_2px() {
        let mut a = str_to_vec("abcd");
        let mut b = str_to_vec("wxyz");
        crossover_kpx(&mut a, &mut b, &[1, 2]);
        assert_eq!(vec_to_str(&a), "axcd");
        assert_eq!(vec_to_str(&b), "wbyz");
    }

    #[test]
    fn test_crossover_ux() {
        let mut r = StepRng::new(1 << 31, 1 << 31);
        let mut a = str_to_vec("abcd");
        let mut b = str_to_vec("wxyz");
        crossover_ux(&mut a, &mut b, &mut r);
        assert_eq!(vec_to_str(&a), "wbyd");
        assert_eq!(vec_to_str(&b), "axcz");
    }
}
