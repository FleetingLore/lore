use std::io;
use std::fs;
use std::path::Path;

mod lir;
use lir::root::*;

/// 文件类，作为数据的所有者
struct File {
    path: String,
    content: Vec<String>,
}

impl File {
    fn new(file_path: &str) -> Self {
        if !Path::new(file_path).exists() {
            panic!("文件不存在: {}", file_path);
        }

        let content_str = fs::read_to_string(file_path).expect("读取文件失败");
        let content: Vec<String> = content_str
            .as_str()
            .lines()
            .map(|line| line.to_string())
            .collect();

        File {
            path: file_path.to_string(),
            content,
        }
    }

    fn parse(self) -> Root<'static> {
        let lines: Vec<&'static str> = self
            .content
            .into_iter()
            .map(|s| {
                let boxed_str = s.into_boxed_str();
                let leaked: &'static mut str = Box::leak(boxed_str);
                &*leaked
            })
            .collect();

        parse_root(lines)
    }
}

fn read_and_parse_file(file_path: &str) -> Root<'static> {
    File::new(file_path).parse()
}

fn main() {
    println!("[{:?}]", std::env::current_dir().unwrap());

    loop {
        println!("_");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.as_str().trim();

        let root = read_and_parse_file(input);
        println!("{}", "-".repeat(40));
        println!("{}", root);
        println!("{}", "-".repeat(40));
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File: {} ({} lines)", self.path, self.content.len())
    }
}
