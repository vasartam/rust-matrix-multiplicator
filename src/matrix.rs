use std::iter::Iterator;

pub(crate) struct Matrix {
    pub(crate) rows: Vec<Vec<i32>>,
}

impl Matrix {
    pub(crate) fn print_row(row: &Vec<i32>) -> String {
        return format!("[{}]", row.iter().map(|&cell| format!("{: >4}", cell)).collect::<Vec<String>>().join(", "));
    }
}

