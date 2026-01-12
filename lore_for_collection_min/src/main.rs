mod line;
mod parser;
mod input_lore;
mod output;

use std::env;
use std::path::Path;

fn main() {
    // 接收命令行参数
    let args: Vec<String> = env::args().collect();

    // 从命令行参数解析输入文件路径和输出文件路径
    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);

    // 读取文件
    let content: String = input_lore::input_lore_file(input_path);

    // 解析文件
    let target = input_lore::parse(content);

    // 生成 html 目标文件
    output::output_html("Test", target, output_path);
}
