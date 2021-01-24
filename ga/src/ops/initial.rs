pub fn rand_vec<T>(k: usize, mut f: impl FnMut() -> T) -> Vec<T> {
    (0..k).map(|_| f()).collect()
}

pub fn vec_to_str(input: &[char]) -> String {
    input.iter().collect()
}

pub fn str_to_vec(input: &str) -> Vec<char> {
    input.chars().collect()
}
