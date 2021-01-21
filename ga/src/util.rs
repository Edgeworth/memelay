use rand::Rng;
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
pub fn crossover_ux<O: FromIterator<T::Item>, T: IntoIterator, R: Rng>(
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
    use rand::rngs::mock::StepRng;

    use super::*;

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
}
