fn main_() {
    let mut front: Grid = [[0; COLS]; ROWS];
    let mut back: Grid = [[0; COLS]; ROWS];

    initialize(&mut front, Init::Seeded(999));

    if FFWD {
        let max = 10_000;

        for i in 0..=max {
            display(&front);
            next_generation(&front, &mut back);
            front.copy_from_slice(&back);
            println!("{}/{}", i, max);
            print!("\x1B[{}A", ROWS + 1);
        }
    }

    loop {
        display(&front);
        next_generation(&front, &mut back);
        front.copy_from_slice(&back);
        print!("\x1B[{}A", ROWS);
        thread::sleep(Duration::from_millis(100));
    }
}

use rand::prelude::*;
use std::{thread, time::Duration};

const FFWD: bool = true;
const ROWS: usize = 32;
const COLS: usize = ROWS * 2;

type Grid = [[u8; COLS]; ROWS];

#[allow(unused)]
enum Init {
    Glider,
    Random,
    Seeded(u64),
}

fn initialize(grid: &mut Grid, init_type: Init) {
    let mut randomize = |mut rng: StdRng| {
        for row in grid.iter_mut() {
            for cell in row.iter_mut() {
                *cell = if rng.gen_bool(0.5) { 1 } else { 0 };
            }
        }
    };

    match init_type {
        Init::Glider => {
            grid[0][1] = 1;
            grid[1][2] = 1;
            grid[2][0] = 1;
            grid[2][1] = 1;
            grid[2][2] = 1;
        }
        Init::Random => {
            randomize(rand::rngs::StdRng::from_entropy());
        }
        Init::Seeded(seed) => {
            randomize(rand::rngs::StdRng::seed_from_u64(seed));
        }
    }
}

fn display(grid: &Grid) {
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{}", if cell == 1 { '#' } else { '.' });
        }
        println!();
    }
}

fn next_generation(front: &Grid, back: &mut Grid) {
    for y in 0..ROWS {
        for x in 0..COLS {
            let neighbors = count_neighbors(front, x, y);
            back[y][x] = match (front[y][x], neighbors) {
                (1, 2) | (1, 3) => 1,
                (1, _) => 0,
                (0, 3) => 1,
                _ => 0,
            };
        }
    }
}

fn count_neighbors(grid: &Grid, cx: usize, cy: usize) -> u8 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let x = (cx as isize + dx).rem_euclid(COLS as isize) as usize;
            let y = (cy as isize + dy).rem_euclid(ROWS as isize) as usize;
            count += grid[y][x];
        }
    }
    count
}
