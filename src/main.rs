mod config;
mod matrix;
mod multiplication;
mod matrix_generator;
mod board;

use config::{MATRIX_SIZE_A_WIDTH_AND_B_HEIGHT, MATRIX_SIZE_A_HEIGHT, MATRIX_SIZE_B_WIDTH};
use matrix_generator::MatrixGenerator;
use multiplication::multiply;
use board::print_board;

fn main() {
    let matrix_a = MatrixGenerator::generate(MATRIX_SIZE_A_WIDTH_AND_B_HEIGHT, MATRIX_SIZE_A_HEIGHT);
    let matrix_b = MatrixGenerator::generate(MATRIX_SIZE_B_WIDTH, MATRIX_SIZE_A_WIDTH_AND_B_HEIGHT);
    let matrix_c = multiply(&matrix_a, &matrix_b);

    print_board(&matrix_a, &matrix_b, &matrix_c);
}

