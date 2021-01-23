use std::iter::Iterator;

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
