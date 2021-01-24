use rand::Rng;
use std::iter::Iterator;

// Roulette wheel selection:
pub fn rws<R: Rng + ?Sized>(w: &[f64], r: &mut R) -> Option<usize> {
    multi_rws(w, 1, r).get(0).copied()
}

pub fn multi_rws<R: Rng + ?Sized>(w: &[f64], k: usize, r: &mut R) -> Vec<usize> {
    let sum = w.iter().sum();
    if sum == 0.0 {
        return vec![];
    }

    let mut idxs = Vec::new();
    for _ in 0..k {
        let cursor = r.gen_range(0.0..sum);
        let mut cursum = 0.0;
        for (i, v) in w.iter().enumerate() {
            cursum += v;
            if cursum >= cursor {
                idxs.push(i);
                break;
            }
        }
    }
    idxs
}

// Stochastic universal sampling:
pub fn sus<R: Rng + ?Sized>(w: &[f64], k: usize, r: &mut R) -> Vec<usize> {
    let sum: f64 = w.iter().sum();
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_rws() {
        let mut r = StepRng::new(1 << 31, 1 << 31);
        assert_eq!(rws(&[], &mut r), None);
        assert_eq!(rws(&[1.0], &mut r), Some(0));
        assert_eq!(rws(&[0.0, 1.0], &mut r), Some(1));
    }

    #[test]
    fn test_multi_rws() {
        let mut r = StepRng::new(1 << 31, 1 << 31);
        assert_eq!(multi_rws(&[], 0, &mut r), []);
        assert_eq!(multi_rws(&[], 1, &mut r), []);
        assert_eq!(multi_rws(&[1.0], 0, &mut r), []);
        assert_eq!(multi_rws(&[1.0], 1, &mut r), [0]);
        assert_eq!(multi_rws(&[1.0], 1, &mut r), [0]);
        assert_eq!(multi_rws(&[0.0, 1.0], 1, &mut r), [1]);
    }

    #[test]
    fn test_sus() {
        let mut r = StepRng::new(1 << 31, 1 << 31);
        assert_eq!(sus(&[], 0, &mut r), []);
        assert_eq!(sus(&[], 1, &mut r), []);
        assert_eq!(sus(&[1.0], 0, &mut r), []);
        assert_eq!(sus(&[1.0], 1, &mut r), [0]);
        assert_eq!(sus(&[1.0], 1, &mut r), [0]);
        assert_eq!(sus(&[1.0, 1.0], 1, &mut r), [0]);
        assert_eq!(sus(&[0.0, 1.0], 1, &mut r), [1]);
        assert_eq!(sus(&[1.0, 1.0], 2, &mut r), [0, 1]);
        assert_eq!(sus(&[1.0, 2.0], 3, &mut r), [0, 1, 1]);
    }
}
