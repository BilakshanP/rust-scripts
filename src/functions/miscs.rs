pub fn string_to_vec(string: &str) -> Vec<u8> {
    string.bytes().collect()
}

pub fn vec_to_string(vector: &[u8]) -> String {
    String::from_utf8_lossy(vector).into_owned()
}