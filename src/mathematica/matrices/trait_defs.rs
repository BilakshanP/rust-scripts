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