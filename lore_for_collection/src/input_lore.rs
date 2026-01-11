use std::fs;
use std::path::Path;

pub fn input_lore_file_with_file_name(input_lore_file_path: &str) -> String {
    fs::read_to_string(input_lore_file_path).unwrap()
}

// 提取文件名（不含路径和扩展名）
pub fn extract_filename(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("local")
        .to_string()
}
