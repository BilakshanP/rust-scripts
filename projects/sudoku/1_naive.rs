fn print(arr: [[u8; 9]; 9]) {
    for i in arr {
        for j in i {
            print!("{} ", j)
        }

        println!()
    }
}

fn is_safe(grid: &[[u8; 9]; 9], num: u8, row: usize, col: usize) -> bool {
    for i in 0..9 {
        if grid[row][i] == num {
            return false;
        }
    }

    for i in grid {
        if i[col] == num {
            return false;
        }
    }

    let start_row = row - row % 3;
    let start_col = col - col % 3;

    for i in 0..3 {
        for j in 0..3 {
            if grid[i + start_row][j + start_col] == num {
                return false;
            }
        }
    }

    true
}

fn solve_sudoku(grid: &mut [[u8; 9]; 9], mut row: usize, mut col: usize) -> bool {
    if row == 8 && col == 9 {
        return true;
    }

    if col == 9 {
        row += 1;
        col = 0;
    }

    if grid[row][col] > 0 {
        return solve_sudoku(grid, row, col + 1);
    }

    for num in 1..=9 {
        if is_safe(grid, num, row, col) {
            grid[row][col] = num;

            if solve_sudoku(grid, row, col + 1) {
                return true;
            }
        }

        grid[row][col] = 0
    }

    false
}

fn main() {
    let mut  grid = [
        [3, 0, 6, 5, 0, 8, 4, 0, 0],
        [5, 2, 0, 0, 0, 0, 0, 0, 0],
        [0, 8, 7, 0, 0, 0, 0, 3, 1],
        [0, 0, 3, 0, 1, 0, 0, 8, 0],
        [9, 0, 0, 8, 6, 3, 0, 0, 5],
        [0, 5, 0, 0, 9, 0, 6, 0, 0],
        [1, 3, 0, 0, 0, 0, 2, 5, 0],
        [0, 0, 0, 0, 0, 0, 0, 7, 4],
        [0, 0, 5, 2, 0, 6, 3, 0, 0]
    ];

    if solve_sudoku(&mut grid, 0, 0) {
        print(grid)
    } else {
        println!("No solution...")
    }
}