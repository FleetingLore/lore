use std::path::Path;
use crate::input_lore::input_lore_file;
use crate::input_lore::parse;

mod line;
mod parser;
mod input_lore;

fn main() {
    let content: String = input_lore_file(Path::new("..\\test\\basic.lore"));
    let target = parse(content);
    
    for line in &target {
        println!("{}", line);
    }
}
