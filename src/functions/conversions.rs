pub fn str_to_vec(string: &str) -> Vec<u8> {
    string.into()
}

pub fn vec_to_str(vector: &[u8]) -> String {
    let mut out_str = String::with_capacity(vector.len());

    for character in vector {
        out_str.push(*character as char)
    }

    out_str
}