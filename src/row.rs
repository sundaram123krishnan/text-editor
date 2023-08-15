#[derive(Debug)]

pub struct Row {
    pub string: String,
}

impl Row {
    pub fn slice(s: &str) -> Self {
        Self {
            string: String::from(s),
        }
    }
}
