use std::iter::Iterator;

// Calculating fitnesses:
pub fn count_different<T: PartialEq>(s1: &[T], s2: &[T]) -> usize {
    let min = s1.len().min(s2.len());
    let max = s1.len().max(s2.len());
    (0..min).map(|i| if s1[i] != s2[i] { 1 } else { 0 }).fold(max - min, |a, b| a + b)
}

pub fn euclidean_dist(s1: &[f64], s2: &[f64]) -> f64 {
    let max = s1.len().max(s2.len());
    let mut dist = 0.0;
    for i in 0..max {
        let a = s1.get(i).unwrap_or(&0.0);
        let b = s2.get(i).unwrap_or(&0.0);
        dist += (a - b) * (a - b);
    }
    dist.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_different() {
        assert_eq!(count_different(&[1], &[1]), 0);
        assert_eq!(count_different(&[1], &[2]), 1);
        assert_eq!(count_different(&[1], &[1, 2]), 1);
        assert_eq!(count_different(&[1, 2], &[1]), 1);
    }
}
