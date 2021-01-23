use std::iter::Iterator;

// Calculating fitnesses:
pub fn count_different<T: PartialEq>(s1: &[T], s2: &[T]) -> usize {
    let min = s1.len().min(s2.len());
    let max = s1.len().max(s2.len());
    (0..min).map(|i| if s1[i] != s2[i] { 1 } else { 0 }).fold(max - min, |a, b| a + b)
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

    #[test]
    fn test_count_different() {
        assert_eq!(count_different(&[1], &[1]), 0);
        assert_eq!(count_different(&[1], &[2]), 1);
        assert_eq!(count_different(&[1], &[1, 2]), 1);
        assert_eq!(count_different(&[1, 2], &[1]), 1);
    }
}
