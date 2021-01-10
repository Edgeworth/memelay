pub fn crossover_vec<T: Clone>(a: &[T], b: &[T], crosspoint: usize) -> Vec<T> {
    let mut v = Vec::new();
    v.extend(a[..crosspoint].iter().cloned());
    v.extend(b[crosspoint..].iter().cloned());
    v
}
