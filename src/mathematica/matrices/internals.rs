use super::prelude::*;

pub mod initialization {
    use super::*;

    pub fn new_scalar_matrix_internal(size: usize, scalar: f64) -> Info<Mat> {
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

        Value(Some(out_mat))
    }

    #[allow(clippy::type_complexity)]
    pub fn scalar_bundle(size: usize, scalar: f64, determinat: f64) -> (Info<Mat>, Info<Mat>, Info<Mat>, Info<Mat>) {
        let minors: Info<Mat> =    new_scalar_matrix_internal(size, determinat);
        let cofactors: Info<Mat> = new_scalar_matrix_internal(size, determinat);
        let adjoint: Info<Mat> =   new_scalar_matrix_internal(size, determinat);
        let inverse: Info<Mat> =   new_scalar_matrix_internal(size, 1.0 / scalar);

        println!("{:?}", inverse);

        (minors, cofactors, adjoint, inverse)
    }
}

pub mod get_set {
    use super::*;

    #[inline(never)]
    pub fn calc_transpose(mat: &Mat, row: usize, col: usize) -> Mat {
        let mut out_mat: Mat = Vec::with_capacity(row);
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(col);

        for row in mat {
            for value in row {
                tmp_vec.push(*value)
            }

            out_mat.push(tmp_vec.clone());
            tmp_vec.clear()
        }

        out_mat
    }

    #[inline(never)]
    fn transpose_if_exists(wrapped_mat: &Info<Mat>, row: usize, col: usize) -> Info<Mat> {
        if let Value(val) = wrapped_mat {
            return if let Some(mat) = val {
                Value(Some(calc_transpose(mat, row, col)))
            } else {
                Value(None)
            }            
        }

        NC
    }

    #[allow(clippy::type_complexity)]
    pub fn transpose_bundle(matrix: &Matrix) -> (Info<Mat>, Info<Mat>, Info<Mat>, Info<Mat>) {
        let (row, col) = matrix.order();

        let minors: Info<Mat> = transpose_if_exists(&matrix.minors, row, col);
        let cofactors: Info<Mat> = transpose_if_exists(&matrix.cofactors, row, col);
        let adjoint: Info<Mat> = transpose_if_exists(&matrix.adjoint, row, col);
        let inverse: Info<Mat> = transpose_if_exists(&matrix.inverse, row, col);

        (minors, cofactors, adjoint, inverse)
    }

    //pub fn minors
}