#![allow(unused, unreachable_code)]

use std::hash::{DefaultHasher, Hasher};
use std::io::Read;

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

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

fn build_markov_chain(corpus: &str, n: usize) -> HashMap<Vec<String>, Vec<String>> {
    let mut markov_chain: HashMap<Vec<String>, Vec<String>> = HashMap::new();
    let words: Vec<String> = corpus.split_whitespace().map(String::from).collect();

    for window in words.windows(n + 1) {
        let key = window[..n].to_vec();
        let value = window[n].to_string();

        markov_chain.entry(key).or_default().push(value);
    }

    markov_chain
}

fn generate_poem(
    markov_chain: &HashMap<Vec<String>, Vec<String>>,
    _n: usize,
    length: usize,
    delimiter: Option<&str>,
) -> String {
    let mut rng = thread_rng();
    let keys: Vec<&Vec<String>> = markov_chain.keys().collect();
    let mut key = keys.choose(&mut rng).unwrap().to_owned().to_vec();
    let mut poem = key.join(" ");
    let mut exceeded_length = false;

    while !exceeded_length || delimiter.is_some() && !poem.ends_with(delimiter.unwrap()) {
        if let Some(next_words) = markov_chain.get(&key) {
            let next_word = next_words.choose(&mut rng).unwrap();
            poem.push(' ');
            poem.push_str(next_word);

            key = key[1..].to_vec();
            key.push(next_word.to_string());

            if poem.split_whitespace().count() >= length {
                exceeded_length = true;
            }
        } else {
            break;
        }
    }

    poem
}

fn main() {
    let binding = std::fs::read_to_string("hashes.txt").unwrap();
    let corpus: &str = binding.as_str();

    let markov_chain = build_markov_chain(corpus, 2);
    let poem = generate_poem(&markov_chain, 2, 50, Some("."));

    println!("{}", poem);

    use hash::sha::sha2::*;

    for i in 0..256 {
        // println!("{}", to_hex(&sha256(&[i as u8])));
    }
}
