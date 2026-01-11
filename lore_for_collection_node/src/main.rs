#![feature(clone_from_ref)]#![feature(clo
ne_from_ref)]#![feature(clo
ne_from_ref)]// src/main.rs


use std::env;
use std::path::Path;
use crate::input_lore::input_lore_file;
use crate::input_lore::parse;
use crate::output::output_html;
use crate::node::calculate_depth;

mod line;
mod parser;
mod input_lore;
mod output;
mod node;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        return;
    }

    let content = input_lore_file(Path::new(&args[1]));
    let target = parse(content.as_str());

    for line in &target {
        println!("{}", line);
    }

    // 将lines转换为树结构
    let root = node::into_root("Root".to_string(), target.as_slice());
    println!("Root has {} top-level nodes", root.nodes.len());

    // 计算深度示例
    for (i, node) in root.nodes.into_iter().enumerate() {
        let depth = calculate_depth(&node, 0);
        println!("Top-level node {} has depth {}", i, depth);
    }

    // 输出HTML
    output_html("Test", &target, Path::new(&args[2]));
}