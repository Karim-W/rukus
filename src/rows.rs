use std::{fs, path, vec::Vec};

pub struct Rows {
    rows: Vec<String>,
}

impl Rows {
    pub fn new() -> Self {
        Self {
            rows: vec![" ".into()],
        }
    }

    pub fn rows_length(&self) -> usize {
        self.rows.len()
    }

    pub fn push(&mut self, at: u16, string_to_append: &str) {
        if at > self.rows_length() as u16 - 1 {
            return;
        }
        self.rows[at as usize].push(string_to_append.chars().next().unwrap());
    }

    pub fn create_line(&mut self) {
        self.rows.push("".into());
    }

    pub fn serve(&self, at: usize) -> &str {
        if at > self.rows_length() - 1 {
            return "";
        }
        &self.rows[at]
    }

    pub fn add_from_file(file_path: &path::Path) -> Self {
        let fyle = fs::read_to_string(file_path).expect("can not read file");
        let rows = fyle.lines().map(|x| x.into()).collect();
        Self { rows }
    }
}
