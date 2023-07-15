use super::prelude::*;
use crate::mathematica::matrices::internals::get_set as gs;

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
        let row: usize = self.row;
        let col: usize = self.col;

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


    pub fn minors(&self) -> Matrix {
        todo!()
    }
}