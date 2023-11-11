use rand::prelude::*;
use std::iter::Iterator;

const MATRIX_SIZE: usize = 4;
const NUMBERS_RANGE_MAX: i32 = 10;
const NUMBERS_RANGE: std::ops::RangeInclusive<i32> = -10..=NUMBERS_RANGE_MAX;

fn main() {
    let matrix_a = MatrixGenerator::generate(MATRIX_SIZE);
    let matrix_b = MatrixGenerator::generate(MATRIX_SIZE);

    MatrixMultiplicator::print(&matrix_a, &matrix_b);
}

struct MatrixMultiplicator;
impl MatrixMultiplicator {
    fn print(matrix_a: &Matrix, matrix_b: &Matrix) {
        let mut output: String = "".to_string();

        let matrix_c = MatrixMultiplicator::multiply(matrix_a, matrix_b);
        let mut matrix_c_iter = matrix_c.rows.iter();

        let mut matrix_a_iter = matrix_a.rows.iter();
        let mut matrix_b_iter = matrix_b.rows.iter();

        // Get output of first row from matrix B.
        let matrix_a_first_row = Matrix::print_row(&matrix_a.rows[0]);
        let matrix_a_width = matrix_a_first_row.len();

        let gap_vertical = 1;
        let gap_horizontal = 3;

        // Print matrix B with left offset that's equal to matrix A width.
        while let Some(matrix_b_row) = matrix_b_iter.next() {
            // Left offset.
            output = format!("{}{}", output, " ".repeat(matrix_a_width + gap_horizontal));

            // Matrix B row.
            output = format!("{}{}", output, Matrix::print_row(matrix_b_row));

            // Line ending.
            output = format!("{}\n", output);
        }

        // Vertical gap.
        output = format!("{}{}", output, "\n".repeat(gap_vertical));

        // Print matrix A and the matrix C.
        loop {
            // Matrix A row.
            match matrix_a_iter.next() {
                Some(matrix_a_row) => {
                    output = format!("{}{}", output, Matrix::print_row(matrix_a_row));
                }
                None => { break; }
            }

            // Gap.
            output = format!("{}{}", output, " ".repeat(gap_horizontal));

            // Matrix C row.
            match matrix_c_iter.next() {
                Some(matrix_c_row) => {
                    output = format!("{}{}", output, Matrix::print_row(matrix_c_row));
                }
                None => { break; }
            }

            output = format!("{}\n", output);
        }

        print!("{}", output);
    }

    fn multiply(matrix_a: &Matrix, matrix_b: &Matrix) -> Matrix {
        let matrix_a_height = matrix_a.rows.len();
        let matrix_b_width = matrix_b.rows[0].len();

        let mut rows = Vec::with_capacity(matrix_a_height);
        for y in 0..matrix_a_height {
            let mut row = Vec::with_capacity(matrix_b_width);
            for x in 0..matrix_b_width {
                let cell_result = MatrixMultiplicator::compute_cell((x, y), matrix_a, matrix_b);
                row.push(cell_result);
            }
            rows.push(row);
        }

        return Matrix {
            rows,
        }
    }

    fn compute_cell(cell_position: (usize, usize), matrix_a: &Matrix, matrix_b: &Matrix) -> i32 {
        let mut result = 0;

        // Matrix A width = matrix B height.
        let n = matrix_b.rows.len();

        let (cell_x, cell_y) = cell_position;
        
        for i in 0..n {
            result += matrix_a.rows[cell_y][i] * matrix_b.rows[i][cell_x];
        }

        return result;
    }
}

struct Matrix {
    rows: Vec<Vec<i32>>,
}

impl Matrix {
    fn print_row(row: &Vec<i32>) -> String {
        return format!("[{}]", row.iter().map(|&cell| format!("{: >4}", cell)).collect::<Vec<String>>().join(", "));
    }
}

struct MatrixGenerator;
impl MatrixGenerator {
    fn generate(size: usize) -> Matrix {
        let mut rng = rand::thread_rng();

        let mut rows = Vec::with_capacity(size);
        for _i in 0..size {
            let mut row = Vec::with_capacity(size);
            for _j in 0..size {
                row.push(rng.gen_range(NUMBERS_RANGE));
            }
            rows.push(row);
        }

        let matrix = Matrix {
            rows,
        };

        return matrix;
    }
}

#[cfg(test)]
mod tests {
    use crate::{MatrixMultiplicator, Matrix};

    #[test]
    fn compute_cell_simple() {
        let matrix_a = Matrix {
            rows: vec![
                vec![3, 5],
            ],
        };

        let matrix_b = Matrix {
            rows: vec![
                vec![4],
                vec![2],
            ],
        };

        let compute_result = MatrixMultiplicator::compute_cell((0, 0), &matrix_a, &matrix_b);
        assert_eq!(compute_result, 22);
    }

    #[test]
    fn compute_cell_complex() {
        let matrix_a = Matrix {
            rows: vec![
                vec![3, 5, 7],
                vec![2, 9, 4],
            ],
        };

        let matrix_b = Matrix {
            rows: vec![
                vec![1, 6],
                vec![4, 2],
                vec![4, 8],
            ],
        };

        assert_eq!(MatrixMultiplicator::compute_cell((0, 0), &matrix_a, &matrix_b), 51);
        assert_eq!(MatrixMultiplicator::compute_cell((0, 1), &matrix_a, &matrix_b), 54);
        assert_eq!(MatrixMultiplicator::compute_cell((1, 0), &matrix_a, &matrix_b), 84);
        assert_eq!(MatrixMultiplicator::compute_cell((1, 1), &matrix_a, &matrix_b), 62);
    }
}

// TODO: split code into multiple files and modules.
// TODO: commit to git repo

