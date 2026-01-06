pub fn parse_root(raw_lines: Vec<&str>) -> Root<'_> {
    let data = raw_lines
        .into_iter()
        .map(parse_line)
        .collect();

    Root { data }
}
