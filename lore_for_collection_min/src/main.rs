use std::path::Path;
use crate::input_lore::input_lore_file;
use crate::input_lore::parse;
use crate::output::output_html;

mod line;
mod parser;
mod input_lore;
mod output;

fn main() {
    let content: String = input_lore_file(Path::new("..\\test\\basic.lore"));
    let target = parse(content);
    
    for line in &target {
        println!("{}", line);
    }
    
    output_html("Test", target, Path::new("..\\test\\test.html"));
    
}
