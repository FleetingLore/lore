use std::{env, fs};
use lore_for_collection::input_lore::{extract_filename, input_lore_file_with_file_name};
use lore_for_collection::output_html::generate_html;
use lore_for_collection::parse::{into_nodes, parse_lines};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input_lore_file> <output_html_file>", args[0]);
        return;
    }

    let input_lore_file_path: &str = &args[1];
    let output_html_file_path: &str = &args[2];

    let input_lore_file: String = input_lore_file_with_file_name(input_lore_file_path);

    let lines = parse_lines(input_lore_file.as_str());
    let nodes = into_nodes(&lines);
    let html = generate_html(nodes, extract_filename(output_html_file_path));

    if let Err(e) = fs::write(output_html_file_path, html) {
        eprintln!("Error writing to file {}: {}", output_html_file_path, e);
        return;
    }

    println!("done from {} to {}", input_lore_file_path, output_html_file_path);
}
