use std::{env};
use std::path::Path;
use crate::input_lore::input_lore_file;
use crate::input_lore::parse;
use crate::output::output_html;

mod line;
mod parser;
mod input_lore;
mod output;

fn main() {
    let args: Vec<String> = env::args().collect();

    let content: String = input_lore_file(Path::new(&args[1]));
    let target = parse(content);

    for line in &target {
        println!("{}", line);
    }

    output_html("Test", target, Path::new(&args[2]));

}
