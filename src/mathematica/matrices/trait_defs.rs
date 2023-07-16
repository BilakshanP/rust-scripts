use super::prelude::*;

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut printable: String = String::new();

        for row in self.mat.iter() {
            printable.push_str("( ");

            for val in row.iter() {
                printable += &format!("{} ", val);
            }

            printable.push_str(")\n");
        }
        
        write!(f, "{}", printable)
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Self) -> Self::Output {
        let (row, col): (usize, usize) = self.order();
        let order_2: (usize, usize) = other.order();

        if (row, col) != order_2 {
            panic!("Can't subtract matrices, mismatched order, lhs: {:?} and rhs: {:?}", (row, col), order_2)
        }

        let mat_1: Mat = self.mat;
        let mat_2: Mat = other.mat;

        let mut out_mat: Mat = Vec::with_capacity(row);
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(col);

        for (i1, j1) in mat_1.iter().enumerate() {
            for (i2, j2) in j1.iter().enumerate() {
                tmp_vec.push(j2 + mat_2[i1][i2])
            }

            out_mat.push(tmp_vec.clone());
            tmp_vec.clear();
        }

        println!("improve addition");

        Self::new_lazy(out_mat, row, col)
    }
}

impl std::ops::Sub for Matrix {
    type Output = Matrix;

    fn sub(self, other: Self) -> Self::Output {
        let (row, col): (usize, usize) = self.order();
        let order_2: (usize, usize) = other.order();

        if (row, col) != order_2 {
            panic!("Can't subtract matrices, mismatched order, lhs: {:?} and rhs: {:?}", (row, col), order_2)
        }

        let mat_1: Mat = self.mat;
        let mat_2: Mat = other.mat;

        let mut out_mat: Mat = Vec::with_capacity(row);
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(col);

        for (i1, j1) in mat_1.iter().enumerate() {
            for (i2, j2) in j1.iter().enumerate() {
                tmp_vec.push(j2 - mat_2[i1][i2])
            }

            out_mat.push(tmp_vec.clone());
            tmp_vec.clear();
        }

        println!("improve subtraction");

        Self::new_lazy(out_mat, row, col)
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Self::Output {
        let (row_1, col_1): (usize, usize) = self.order();
        let (row_2, col_2): (usize, usize) = other.order();

        if col_1 != row_2 {
            panic!("Can't multiply matrices, mismatched column-row pair, lhs column: {} and rhs row: {}", col_1, row_2)
        }

        let mat_1: Mat = self.mat;
        let mat_2: Mat = other.mat;

        let mut out_mat: Mat = Vec::new();
        let mut tmp_vec: Vec<f64> = Vec::new();
        let mut sum_vec: Vec<f64> = Vec::new();

        for i in mat_1.iter() {
            for j in 0..col_2 {
                for k in 0..row_2 {
                    sum_vec.push(i[k] * mat_2[k][j])
                }

                tmp_vec.push(sum_vec.iter().sum());
                sum_vec.clear();
            }

            out_mat.push(tmp_vec.clone());
            tmp_vec.clear();
        }

        Self::new_lazy(out_mat, col_2, row_1)
    }
}

// fn print(mat: Vec<Vec<f64>>) {
//     let divided_vec = mat.iter()
//         .map(
//             |inner_vec| {
//                 inner_vec.iter()
//                     .map(
//                         |num| {
//                             (format!("{:?}", num.trunc()),
//                             format!("{:?}", num.fract()))
//                         }
//                     )
//                     .collect::<Vec<(_, _)>>()
//             }
//         )
//         .collect::<Vec<Vec<(_, _)>>>();
// 
//     println!("\n{:?}", divided_vec)
// }