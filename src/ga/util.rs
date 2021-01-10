pub fn crossover_vec<T: Clone>(a: &[T], b: &[T], crosspoint: usize) -> Vec<T> {
    let mut v = Vec::new();
    v.extend(a[..crosspoint].iter().cloned());
    v.extend(b[crosspoint..].iter().cloned());
    v
}

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
