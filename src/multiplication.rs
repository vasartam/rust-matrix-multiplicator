use crate::matrix::Matrix;

pub(crate) fn multiply(matrix_a: &Matrix, matrix_b: &Matrix) -> Matrix {
    let matrix_a_height = matrix_a.rows.len();
    let matrix_b_width = matrix_b.rows[0].len();

    let mut rows = Vec::with_capacity(matrix_a_height);
    for y in 0..matrix_a_height {
        let mut row = Vec::with_capacity(matrix_b_width);
        for x in 0..matrix_b_width {
            let cell_result = compute_cell((x, y), matrix_a, matrix_b);
            row.push(cell_result);
        }
        rows.push(row);
    }

    return Matrix {
        rows,
    }
}

pub(crate) fn compute_cell(cell_position: (usize, usize), matrix_a: &Matrix, matrix_b: &Matrix) -> i32 {
    let mut result = 0;

    // Matrix A width = matrix B height.
    let n = matrix_b.rows.len();

    let (cell_x, cell_y) = cell_position;
    
    for i in 0..n {
        result += matrix_a.rows[cell_y][i] * matrix_b.rows[i][cell_x];
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use super::compute_cell;

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

        let compute_result = compute_cell((0, 0), &matrix_a, &matrix_b);
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

        assert_eq!(compute_cell((0, 0), &matrix_a, &matrix_b), 51);
        assert_eq!(compute_cell((0, 1), &matrix_a, &matrix_b), 54);
        assert_eq!(compute_cell((1, 0), &matrix_a, &matrix_b), 84);
        assert_eq!(compute_cell((1, 1), &matrix_a, &matrix_b), 62);
    }
}

