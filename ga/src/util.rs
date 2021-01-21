use smallvec::SmallVec;
use std::iter::{FromIterator, Iterator};

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
// pub fn crossover_ux
// pub fn crossover_kpx<'a, I: Clone, O: FromIterator<I>>(
//     mut s1: &'a [I],
//     mut s2: &'a [I]) -> O {

// }
// Mutation:

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
    fn test_crossover_kpx() {
        assert_eq!(
            crossover_kpx::<String, _>("abcd".chars(), "wxyz".chars(), &[3]),
            ("abcz".to_string(), "wxyd".to_string())
        );
    }
}
