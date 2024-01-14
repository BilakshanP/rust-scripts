#![allow(unused)]

use std::collections::HashSet;

use num::Integer;

pub mod macros;

module!( pub base, hash, enc_dec, functions, data_structures, algorithms, mathematica );

fn main() {
    let board: Board = [[[[0; 3]; 3]; 3]; 3];

    println!("{:#?}", board);
}

type Board = [[[[u8; 3]; 3]; 3]; 3];

fn draw_board(board: Board) {
    // println!("   {} | {} | {}  ||  {} | {} | {}  ||  {} | {} | {}   ", board[0][0][])
}
