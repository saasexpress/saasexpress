use rand::{Rng, distr::Alphanumeric};

pub fn generate_random_id(length: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
