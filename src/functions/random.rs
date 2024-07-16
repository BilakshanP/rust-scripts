use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

/// inclusive
pub fn gen_random<T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform>(
    start: T,
    end: T,
    mut thread: ThreadRng,
) -> T {
    thread.gen_range(start..=end)
}

/// inclusize
pub fn generate_random_number(start: i32, end: i32) -> i32 {
    rand::thread_rng().gen_range(start..=end)
}

/// inclusive
pub fn generate_random_float(start: f64, end: f64) -> f64 {
    rand::thread_rng().gen_range(start..=end)
}

/// inclusive
pub fn gen_usize(start: usize, end: usize) -> usize {
    gen_random(start, end, rand::thread_rng())
}

/// inclusive
pub fn randomized_list(start: usize, end: usize, n: usize) -> Vec<Vec<usize>> {
    let mut rng: ThreadRng = thread_rng();
    let mut result: Vec<Vec<usize>> = Vec::new();

    let original_list: Vec<usize> = (start..=end).collect();

    for _ in 0..n {
        let mut randomized_list: Vec<usize> = original_list.clone();
        randomized_list.shuffle(&mut rng);
        result.push(randomized_list);
    }

    result
}

pub fn vote_result(vote_sample: Vec<Vec<usize>>) -> Vec<isize> {
    let len: usize = vote_sample[0].len();

    let mut tier: Vec<isize> = {
        let lb2: isize = (len / 2) as isize;
        let i: std::ops::RangeInclusive<isize> = -lb2..=lb2;

        match len % 2 {
            0 => i.filter(|&value| value != 0).collect(),
            1 => i.collect(),
            _ => unreachable!(),
        }
    };

    tier.reverse();

    let mut result: Vec<isize> = vec![0; len];

    for vote in vote_sample {
        for (index, &value) in vote.iter().enumerate() {
            result[value] += tier[index];
        }
    }

    result
}

use std::collections::HashMap;

pub fn calculate_char_frequency(input_string: &str) -> Vec<(char, usize)> {
    let mut char_frequency: HashMap<char, usize> = HashMap::new();

    // Count the frequency of each character in the input string
    for c in input_string.chars() {
        let counter = char_frequency.entry(c).or_insert(0);
        *counter += 1;
    }

    println!("{:?}", char_frequency);

    // Convert the HashMap to a vector of tuples and sort by characters
    let mut char_frequency_vec: Vec<(char, usize)> = char_frequency.into_iter().collect();
    char_frequency_vec.sort_by(|a, b| a.0.cmp(&b.0));

    char_frequency_vec
}

/*
fn add_tag(list: Vec<Vec<usize>>) -> Vec<(Vec<usize>, usize)> {
    let mut sum: usize;
    let mut product: usize;

    let mut tagged_list: Vec<(Vec<usize>, usize)> = Vec::with_capacity(list.len());

    for mut sub_list in list {
        let original_sub_list: Vec<usize> = sub_list.clone();

        sub_list.reverse();

        sum = 0;
        product = 1;

        for num in sub_list {
            sum += num * product;
            product *= 10;
        }

        tagged_list.push((original_sub_list, sum))
    }

    tagged_list
}
*/
