mod config;
mod matrix;
mod multiplication;
mod matrix_generator;
mod board;

use config::MATRIX_SIZE;
use matrix_generator::MatrixGenerator;
use multiplication::multiply;
use board::print_board;

fn main() {
    let matrix_a = MatrixGenerator::generate(MATRIX_SIZE);
    let matrix_b = MatrixGenerator::generate(MATRIX_SIZE);
    let matrix_c = multiply(&matrix_a, &matrix_b);

    print_board(&matrix_a, &matrix_b, &matrix_c);
}

