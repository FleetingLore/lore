mod types;
mod structure;
mod content_types;

use structure::data::root::Root;
use structure::parser::root::parse_root;

use std::fs;
use std::path::Path;

pub struct File {
    path: String,
    content: Vec<String>,
}

impl File {
    // 获取文件数据
    pub fn new(file_path: &str) -> Self {
        // 如果文件不存在
        if !Path::new(file_path).exists() {
            println!("文件不存在: {}", file_path);
            return File { path: "".to_string(), content: vec![] }
        }

        // 读取文件
        let content_str = fs::read_to_string(file_path)
            .expect("读取文件失败");
        let content: Vec<String> = content_str
            .as_str()
            .lines()
            .map(|line| line.to_string())
            .collect();

        // 返回文件数据
        File {
            path: file_path.to_string(),
            content,
        }
    }

    // 数据解析
    pub fn parse<'f>(self) -> Root<'f> {
        // 解析每一行
        let lines: Vec<&'f str> = self
            .content
            .into_iter()
            .map(|s| {
                let boxed_str = s.into_boxed_str();
                let leaked: &'f mut str = Box::leak(boxed_str);
                &*leaked
            })
            .collect();

        // 生成 root
        parse_root(lines)
    }
}

// 给文件名，返回解析好的数据
pub fn read_and_parse_file<'f>(file_path: &str) -> Root<'f> {
    File::new(file_path).parse()
}

// 便于检查文件的基本信息
impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File: {} ({} lines)", self.path, self.content.len())
    }
}
