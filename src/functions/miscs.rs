use rand::{Rng, rngs::ThreadRng};

pub fn string_to_vec(string: &str) -> Vec<u8> {
    string.bytes().collect()
}

pub fn vec_to_string(vector: &[u8]) -> String {
    String::from_utf8_lossy(vector).into_owned()
}

pub fn generate_random_number(start: i32, end: i32) -> i32 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(start..=end)
}

pub fn generate_random_float(start: f64, end: f64) -> f64 {
    let mut rng: ThreadRng = rand::thread_rng();
    rng.gen_range(start..=end)
}

pub fn gen_random<T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform>(start: T, end: T, mut thread: ThreadRng) -> T {
    thread.gen_range(start..=end)
}