const PRINTABLES: [char; 95] = [
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*',
    '+', ',', '-', '.', '/', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', ':', ';', '<', '=', '>',
    '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\',
    ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    '{', '|', '}', '~', ' '
];

use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    let parent_str = "string";
    let parent_vec: Vec<char> = parent_str.chars().collect();
    let population_size = 50;
    let generations = i32::MAX;

    let mut population: Vec<Vec<char>> = vec![Vec::with_capacity(parent_vec.len()); population_size];

    // Initialize the population with random strings
    for i in population.iter_mut() {
        for _ in 0..parent_vec.len() {
            i.push(*PRINTABLES.choose(&mut rng).unwrap());
        }
    }

    for gen in 0..generations {
        // Evaluate fitness for each individual in the population
        let mut res: Vec<(Vec<char>, i16)> = population.iter()
            .map(|val| {
                let v = compare(val, &parent_vec);
                (val.clone(), v)
            })
            .collect();

        // Sort by fitness (lower fitness is better)
        res.sort_by_key(|x| x.1);



        // Print the top 3 individuals for each generation
        println!("Generation {}", gen);
        for i in 0..3 {
            println!("{:?} -> {}", res[i].0, res[i].1);
        }
        println!("Average: {}\n", res.iter().fold(0, |acc, (_, n)| acc + *n) as usize / res.len());

        // Check if the target string is found
        if res[0].1 == 0 {
            println!("Target string found: {:?}", res[0].0);
            break;
        }

        // Select parents for the next generation using a tournament selection
        let parents: Vec<Vec<char>> = tournament_selection(&res, 20, 5);

        // Perform crossover to create new individuals
        let mut new_population = crossover(&parents, population_size - parents.len(), &mut rng);

        // Apply mutation to the new individuals
        mutate(&mut new_population, 0.1, &mut rng);

        // Replace the old population with the new one
        population = new_population;
    }
}

fn compare(a: &[char], b: &[char]) -> i16 {
    let a: Vec<i16> = a.iter().map(|&ch| ch as u8 as i16).collect();
    let b: Vec<i16> = b.iter().map(|&ch| ch as u8 as i16).collect();

    a.iter().enumerate().fold(0_i16, |acc, (i, &num)| acc + (b[i] - num).abs())
}

// Tournament selection: Selects n individuals with the best fitness out of k randomly chosen individuals
fn tournament_selection(population: &[(Vec<char>, i16)], n: usize, k: usize) -> Vec<Vec<char>> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| {
        (0..k)
            .map(|_| population.choose(&mut rng).unwrap().0.clone())
            .min_by_key(|individual| compare(individual, &population[0].0))
            .unwrap()
    })
    .collect()
}

// Single-point crossover: Creates new individuals by combining parts of two parents
fn crossover(parents: &[Vec<char>], num_children: usize, rng: &mut ThreadRng) -> Vec<Vec<char>> {
    let mut children = Vec::with_capacity(num_children);
    for _ in 0..num_children {
        let parent1 = parents.choose(rng).unwrap();
        let parent2 = parents.choose(rng).unwrap();
        let crossover_point = rng.gen_range(1..parent1.len());
        let mut child = parent1[..crossover_point].to_vec();
        child.extend_from_slice(&parent2[crossover_point..]);
        children.push(child);
    }
    children
}

// Mutation: Applies random changes to each individual with a given probability
fn mutate(population: &mut [Vec<char>], mutation_rate: f64, rng: &mut ThreadRng) {
    for individual in population.iter_mut() {
        for gene in individual.iter_mut() {
            if rng.gen::<f64>() < mutation_rate {
                *gene = *PRINTABLES.choose(rng).unwrap();
            }
        }
    }
}