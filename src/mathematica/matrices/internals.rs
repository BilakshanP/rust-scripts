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

    pub fn determinant(mat: &Mat) -> f64 {
        let size: usize = mat.len();
    
        if size == 2 {
            return mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0];
        }

        let reduced: Vec<Mat> = inner::reduce(mat);
        let flattened: Vec<f64> = inner::flatten(mat);

        let mut tmp_vec: Vec<f64> = Vec::with_capacity(size);

        for i in 0..size {
            tmp_vec.push(
                flattened[i] * determinant(&reduced[i])
            )
        }

        tmp_vec.iter().sum()
    }

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

    pub fn scalar_mul(mat: &Mat, row: usize, col: usize, scalar: f64) -> Mat {
        let mut out_mat: Mat = Vec::with_capacity(row);
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(col);

        for row_vec in mat {
            for val in row_vec {
                tmp_vec.push(val * scalar);
            }

            out_mat.push(tmp_vec.clone());
            tmp_vec.clear();
        }

        out_mat
    }
}

pub mod inner {
    use super::*;

    pub fn trim(mat: &Mat, row: usize, col: usize) -> Vec<Vec<f64>> {
        mat
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != row)
            .map(
                |(_, row_vec)| {
                    row_vec
                        .iter()
                        .enumerate()
                        .filter(|&(j, _)| j != col)
                        .map(|(_, &val)| val)
                        .collect::<Vec<f64>>()
                }
            )
            .collect::<Vec<Vec<f64>>>()
    }
    
    pub fn reduce(mat: &Mat) -> Vec<Mat> {
        (0..mat[0].len())
            .map(|i| trim(mat, 0, i))
            .collect()
    }
    
    pub fn flatten(mat: &[Vec<f64>]) -> Vec<f64> {
        mat[0].iter()
            .enumerate()
            .map(|(i, &value)| value * (-1.0f64).powi(i as i32))
            .collect()
    }
}