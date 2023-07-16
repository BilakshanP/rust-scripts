#![allow(dead_code)]

pub mod old;

mod initialization;
mod trait_defs;
mod get_set;

mod prelude;
mod internals;

use prelude::*;

#[derive(Debug)]
pub struct Matrix {//<T: num::Num + num::pow::Pow<usize, Output = T>> {
    mat: Mat,
    row: usize,
    col: usize,

    det: Info<f64>,
    transpose: Info<Mat>,

    minors: Info<Mat>,
    cofactors: Info<Mat>,
    adjoint: Info<Mat>,
    inverse: Info<Mat>,

    is_square: bool,
    is_identity: Option<bool>,
}