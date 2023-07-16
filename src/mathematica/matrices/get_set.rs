use super::prelude::*;
use crate::mathematica::matrices::internals::{get_set as gs, inner};

impl Matrix {
    /// returns a refernce of inner unwrappd matrix
    pub fn get_mat(&self) -> &Mat {
        &self.mat
    }

    /// returns a clone of inner unwrappd matrix
    pub fn get_mat_clone(&self) -> Mat {
        self.mat.clone()
    }

    /// drops and unwraps the inner matrix
    pub fn get_mat_exhaust(self) -> Mat {
        self.mat
    }

    /// returns order of a matrix
    pub fn order(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    pub fn determinant(&self) -> Result<f64, MatrixError> {
        let mat: &Mat = &self.mat;
        let (row, col) = self.order();

        if row != col {
            return Err( NonSquareMatrix { row, col } );
        }

        if row == 1 {
            return Ok( mat[0][0] );
        }

        if row == 2 {
            return Ok( mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0] )
        }

        let reduced: Vec<Mat> = inner::reduce(mat);
        let flattened: Vec<f64> = inner::flatten(mat);

        let mut tmp_vec: Vec<f64> = Vec::with_capacity(row);

        for i in 0..row {
            tmp_vec.push(flattened[i] * gs::determinant(&reduced[i]))
        }

        Ok( tmp_vec.iter().sum() )
    }


    /// lazy transpose doesn't update the transpose of parent matrix or of return matrix
    pub fn transpose(&self) -> Self {
        let row: usize = self.row;
        let col: usize = self.col;

        if let Some(val) = self.is_identity {
            if val {
                return Self::new_identity_matrix(self.row)
            }
        }

        match &self.transpose {
            Value(val) => match val {
                Some(trans_mat) => {
                    Self::new_lazy(trans_mat.clone(), col, row)
                },
                None => unreachable!(),
            },

            NC => {
                Self::new_lazy(gs::calc_transpose(&self.mat, row, col), col, row)
            },
        }
    }

    /// updates the transpose of parents matrix as well
    pub fn transpose_mut(&mut self) -> Self {
        let row: usize = self.row;
        let col: usize = self.col;

        match &self.transpose {
            Value(val) => match val {
                Some(mat) => {
                    Self::new_lazy(mat.clone(), col, row)
                },
                None => unreachable!(),
            },

            NC => {
                let transpose: Mat = gs::calc_transpose(&self.mat, row, col);
        
                self.transpose = Value(Some(transpose.clone()));

                Self::new_lazy(transpose, col, row)
            },
        }
    }
    /// initializes all fielda of return matrix in accordance to parent matrix
    pub fn transpose_active(&self) -> Self {
        let (row, col) = self.order();

        if let Some(val) = self.is_identity {
            if val {
                return Self::new_identity_matrix(self.row)
            }
        }

        match &self.transpose {
            Value(val) => match val {
                Some(mat) => {
                    Self::new_lazy(mat.clone(), col, row)
                },
                None => unreachable!(),
            },

            NC => {
                let (minors, cofactors, adjoint, inverse) = gs::transpose_bundle(self);

                Self::construct(gs::calc_transpose(&self.mat, row, col), col, row, self.det.clone(), Value(Some(self.mat.clone())), minors, cofactors, adjoint, inverse, self.is_square, self.is_identity)
            },
        }
    }

    /// doesn't update parent, lazy
    pub fn minors(&self) -> Result<Self, MatrixError> {
        let (row, col) = self.order();

        if !self.is_square {
            return Err( NonSquareMatrix { row, col } );
        }

        if let Value(Some(minor)) = self.minors.clone() {
            return Ok( Self::new_lazy(minor, row, col) );
        }

        let (row, col) = self.order();
        let mat: &Mat = &self.mat;
        let mut out_mat: Mat = Vec::with_capacity(row);
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(col);

        for i in 0..row {
            for j in 0..col {
                tmp_vec.push(
                    gs::determinant(&inner::trim(mat, i, j))
                );
            }

            out_mat.push(tmp_vec.clone());
            tmp_vec.clear();
        }

        println!("Improve minors");

        Ok( Self::new_lazy(out_mat, row, col) )
    }

    /// doesn't update parent, lazy
    pub fn cofactors(&self) -> Result<Self, MatrixError> {
        let (row, col) = self.order();

        if !self.is_square {
            return Err( NonSquareMatrix { row, col } );
        }

        if let Value(Some(cofactor)) = self.cofactors.clone() {
            return Ok( Self::new_lazy(cofactor, row, col) );
        }

        let mat: &Mat = &self.minors()?.mat;

        let (row, col) = self.order();
        let mut out_mat: Mat = Vec::with_capacity(row);
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(col);

        for (i1, j1) in mat.iter().enumerate() {
            for (i2, j2) in j1.iter().enumerate() {
                tmp_vec.push(j2 * (-1.0_f64).powi((i1 + i2) as i32));
            }

            out_mat.push(tmp_vec.clone());
            tmp_vec.clear();
        }

        println!("Improve cofactors");

        Ok( Self::new_lazy(out_mat, row, col) )
    }

    /// doesn't update parent, lazy
    pub fn adjoint(&self) -> Result<Self, MatrixError> {
        let (row, col) = self.order();
    
        if !self.is_square {
            return Err( NonSquareMatrix { row, col } );
        }
    
        if let Value(Some(adjoint)) = self.adjoint.clone() {
            return Ok( Self::new_lazy(adjoint, row, col) );
        }
    
        let mat: &Mat = &self.cofactors()?.mat;
        let out_mat: Mat = gs::calc_transpose(mat, row, col);
    
        println!("Improve adjoint");
    
        Ok( Self::new_lazy(out_mat, row, col) )
    }

    /// doesn't update parent, lazy
    pub fn inverse(&self) -> Result<Self, MatrixError> {
        let (row, col) = self.order();
    
        if !self.is_square {
            return Err( NonSquareMatrix { row, col } );
        }

        if let Value(Some(inverse)) = self.inverse.clone() {
            return Ok( Self::new_lazy(inverse, row, col ) );
        }

        let det: f64 = self.determinant()?;

        if det == 0.0 {
            return Err( SingularMatrix );
        }
    
        let mat: Mat = gs::scalar_mul(&self.cofactors().unwrap().get_mat_clone(), row, col, 1.0 / det);

        println!("improve inverse");

        Ok( Self::new_lazy(mat, row, col) )
    }
}