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
        for i in contents.lines() {
            rows.push(Row {
                string: i.to_string(),
            })
        }
        Self { rows }
    }
}
