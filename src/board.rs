use crate::matrix::Matrix;

pub(crate) fn print_board(matrix_a: &Matrix, matrix_b: &Matrix, matrix_c: &Matrix) {
    let mut output: String = "".to_string();

    let mut matrix_a_iter = matrix_a.rows.iter();
    let mut matrix_b_iter = matrix_b.rows.iter();
    let mut matrix_c_iter = matrix_c.rows.iter();

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

