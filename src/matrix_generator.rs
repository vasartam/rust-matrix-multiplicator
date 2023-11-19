use rand::prelude::*;

use crate::matrix::Matrix;
use crate::config::NUMBERS_RANGE;

pub(crate) struct MatrixGenerator;

impl MatrixGenerator {
    pub(crate) fn generate(size: usize) -> Matrix {
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

