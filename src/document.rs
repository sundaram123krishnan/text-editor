use std::fs;

use crate::row::Row;

pub struct Document {
    pub rows: Vec<Row>,
}

impl Document {
    pub fn open() -> Self {
        let mut rows = Vec::new();
        let contents: String = fs::read_to_string("/home/sundaram/main.cpp").unwrap();
        let temp_row = Row { string: contents };
        rows.push(temp_row);
        Self { rows }
    }
}
