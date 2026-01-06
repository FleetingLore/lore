use crate::structure::parser::line::parse_line;
use crate::structure::data::root::Root;

pub fn parse_root(raw_lines: Vec<&str>) -> Root<'_> {
    let indented_lines = raw_lines
        .into_iter()
        .map(parse_line)
        .collect();

    Root { indented_lines }
}
