use std::fs;

use crate::row::Row;

pub struct Document {
    pub rows: Vec<Row>,
}

impl Document {
    pub fn open() -> Self {
        let mut rows = Vec::new();
        let contents = fs::read_to_string("/home/sundaram/Desktop/cpp/main.cpp").unwrap();
        for i in contents.lines() {
            rows.push(Row::slice(i));
        }
        Self { rows }
    }
}
