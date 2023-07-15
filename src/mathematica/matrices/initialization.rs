use super::prelude::*;
use crate::mathematica::matrices::internals::initialization as iz;


impl Matrix {
    #[allow(clippy::too_many_arguments)]
    pub fn construct(mat: Mat, row: usize, col: usize, det: Info<f64>, transpose: Info<Mat>, minors: Info<Mat>, cofactors: Info<Mat>, adjoint: Info<Mat>, inverse: Info<Mat>, is_square: bool, is_identity: Option<bool>) -> Self {
        Self { mat, row, col, det, transpose, minors, cofactors, adjoint, inverse, is_square, is_identity }
    }

    #[inline(always)]
    pub (crate) fn new_lazy(mat: Mat, row: usize, col: usize) -> Self {
        let is_square: bool = row == col;

        Self {
            mat, row, col,
    
            det: NC,
            transpose: NC,
    
            minors: NC,
            cofactors: NC,
            adjoint: NC,
            inverse: NC,

            is_square,
            is_identity: if is_square { None } else { Some(false) }
        }
    }

    pub fn new() -> Self {
        Self::new_lazy(Vec::new(), 0, 0)
    }

    pub fn new_from_vec(mat: Mat) -> Result<Self, MatrixError> {
        let row: usize = mat.len();
        let col: usize = mat[0].len();

        if mat.iter().all(|v: &Vec<f64>| v.len() == col) {
            return Ok( Self::new_lazy(mat, row, col) );
        }
    
        Err( MatrixError::UnEvenRows(col) )
    }

    pub fn new_from_vec_sized(mat: Vec<f64>, row: usize, col: usize) -> Result<Self, MatrixError> {
        let len: usize = mat.len();
        let size: usize = row * col;

        if len != size {
            return Err(MatrixError::MismatchedSize { expected: size, provided: len });
        }

        let mut out_mat: Mat = Vec::with_capacity(row);
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(col);

        for value in mat {
            tmp_vec.push(value);

            if tmp_vec.len() == col {
                out_mat.push(tmp_vec.clone());
                tmp_vec.clear()
            }
        }

        Ok( Self::new_lazy(out_mat, row, col) )
    }

    pub fn new_from_vec_unchecked(mat: Mat) -> Self {
        let row: usize = mat.len();
        let col: usize = mat[0].len();

        Self::new_lazy(mat, row, col)
    }

    pub fn new_from_vec_sized_unchecked(mat: Vec<f64>, row: usize, col: usize) -> Self {
        let mut out_mat: Mat = Vec::with_capacity(row);
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(col);

        for value in mat {
            tmp_vec.push(value);

            if tmp_vec.len() == col {
                out_mat.push(tmp_vec.clone());
                tmp_vec.clear()
            }
        }

        Self::new_lazy(out_mat, row, col)
    }

    pub fn new_identity_matrix(size: usize) -> Self {
        let mut out_mat: Mat = Vec::with_capacity(size);
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(size);

        for i in 0..size {
            for j in 0..size {
                if i == j {
                    tmp_vec.push(1.0);

                    continue;
                } else {
                    tmp_vec.push(0.0)
                }
            }

            out_mat.push(tmp_vec.clone());
            tmp_vec.clear();
        }

        Self {
            mat: out_mat.clone(),

            row: size,
            col: size,
            
            det: Value(Some(1.0)),
            transpose: Value(Some(out_mat.clone())),

            minors: Value(Some(out_mat.clone())),
            cofactors: Value(Some(out_mat.clone())),
            adjoint: Value(Some(out_mat.clone())),
            inverse: Value(Some(out_mat)),
            
            is_square: true,
            is_identity: Some(true)
        }
    }

    pub fn new_scalar_matrix(size: usize, scalar: f64) -> Self {
        let mut out_mat: Mat = Vec::with_capacity(size);
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(size);

        for i in 0..size {
            for j in 0..size {
                if i == j {
                    tmp_vec.push(scalar);

                    continue;
                } else {
                    tmp_vec.push(0.0)
                }
            }

            out_mat.push(tmp_vec.clone());
            tmp_vec.clear();
        }

        Self {
            mat: out_mat.clone(),
            
            row: size,
            col: size,
            
            det: NC,
            transpose: NC,

            minors: NC,
            cofactors: NC,
            adjoint: NC,
            inverse: NC,
        
            is_square: true,
            is_identity: Some(false),
        }
    }

    pub fn new_scalar_matrix_active(size: usize, scalar: f64) -> Self {
        let mut out_mat: Mat = Vec::with_capacity(size);
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(size);

        for i in 0..size {
            for j in 0..size {
                if i == j {
                    tmp_vec.push(scalar);

                    continue;
                } else {
                    tmp_vec.push(0.0)
                }
            }

            out_mat.push(tmp_vec.clone());
            tmp_vec.clear();
        }

        let determinant: f64 = scalar.powi(size as i32);

        let (minors, cofactors, adjoint, inverse) = iz::scalar_bundle(size, scalar, determinant);

        Self {
            mat: out_mat.clone(),
            
            row: size,
            col: size,
            
            det: Value(Some(determinant)),
            transpose: Value(Some(out_mat)),
            minors,
            cofactors,
            adjoint, 
            inverse,

            is_square: true,
            is_identity: Some(false),
        }
    }
}