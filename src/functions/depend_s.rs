pub fn string_to_vec(string: &str) -> Vec<u8> {
    let mut out_vec: Vec<u8> = Vec::new();

    for character in string.bytes() {
        out_vec.push(character)
    }

    out_vec
}

pub fn vec_to_string(vector: &Vec<u8>) -> String {
    let mut out_str: String = String::new();

    for character in vector {
        out_str.push(*character as char)
    }

    out_str
}