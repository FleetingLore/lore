use std::path::Path;
use std::fs;
use crate::line::Line;

// 根据文件路径获取文件
pub fn input_lore_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

// 把文件分成一行一行的然后去除空行
pub fn parse(input: String) -> Vec<Line> {
    input
        .as_str()
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(crate::parser::parse_line)
        .collect()
}
