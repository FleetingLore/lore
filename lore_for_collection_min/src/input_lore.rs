use std::path::Path;
use std::fs;
use crate::line::Line;

pub fn input_lore_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn parse(input: String) -> Vec<Line> {
    input
        .as_str()
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(crate::parser::parse_line)
        .collect()
}
