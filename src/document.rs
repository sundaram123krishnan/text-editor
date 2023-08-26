use std::fs;

use crate::row::Row;

#[derive(Default)]
pub struct Document {
    pub rows: Vec<Row>,
}

impl Document {
    pub fn open(filename: &str) -> Self {
        let mut rows = Vec::new();
        let contents = fs::read_to_string(filename).unwrap();
        let mut row_cnt = 0;
        for i in contents.lines() {
            row_cnt += 1;
            rows.push(Row {
                string: i.to_string(),
                row_nums: row_cnt,
            })
        }
        Self { rows }
    }
}
