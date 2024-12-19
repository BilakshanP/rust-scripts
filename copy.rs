#![allow(unused, unreachable_code)]

mod macros;

module!(
    base,
    hash,
    enc_dec,
    functions,
    data_structures,
    algorithms,
    mathematica
);

use sha2::{Digest, Sha256, Sha512};

use std::{
    fs::File,
    io::{self, Read, Write},
};

fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    let mut buffer = String::new();

    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

fn input_as<T>(prompt: &str, parser: &dyn Fn(String) -> T) -> io::Result<T> {
    let input: String = input(prompt)?;
    Ok(parser(input))
}

fn file_to_string(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn file_to_vec(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();

    file.read_to_end(&mut contents)?;

    Ok(contents)
}

fn string_to_file(path: &str, contents: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

fn vec_to_file(path: &str, contents: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents)?;

    Ok(())
}

fn get_hash(input: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(input.as_bytes());

    let hash = hasher.finalize();

    format!("{:x}", hash)
}

fn get_hash_as_vec(input: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();

    hasher.update(input.as_bytes());

    hasher.finalize().to_vec()
}

fn hexify(input: &[u8]) -> String {
    input.iter().fold(String::new(), |mut acc, x| {
        acc.push_str(&format!("{:02x}", x));
        acc
    })
}

fn unhexify(input: &str) -> Vec<u8> {
    input
        .as_bytes()
        .chunks(2)
        .map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap())
        .collect()
}

fn encrypt(input: &str, password: &str) -> String {
    let hash = get_hash_as_vec(password);

    let input_vec = input.as_bytes().to_vec();

    let mut encrypted = Vec::new();

    for i in 0..input_vec.len() {
        encrypted.push(input_vec[i] ^ hash[i % hash.len()]);
    }

    hexify(&encrypted)
}

fn decrypt(input: &str, password: &str) -> String {
    let hash = get_hash_as_vec(password);

    let encrypted = unhexify(input);

    let mut decrypted = Vec::new();

    for i in 0..encrypted.len() {
        decrypted.push(encrypted[i] ^ hash[i % hash.len()]);
    }

    String::from_utf8(decrypted).unwrap()
}

fn main() {
    let filename = "test.txt"; // input("Enter the filename: ").unwrap();
    let password = "password"; // input("Enter the password: ").unwrap();

    let mode = input("Enter the mode (e/d): ").unwrap();

    let contents = file_to_string(filename).unwrap();

    let result = match mode.trim() {
        "e" => encrypt(&contents, password),
        "d" => decrypt(&contents, password),
        _ => panic!("Invalid mode"),
    };

    string_to_file(filename, &result).unwrap();
}
