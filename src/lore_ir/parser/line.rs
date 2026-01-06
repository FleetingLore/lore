pub fn parse_line(raw_line: &str) -> Line<'_> {
    // 计算缩进
    let indent = raw_line
        .chars()
        .take_while(|&c| c == ' ')
        .count() / 2;

    // 计算 `Content` 数据对象
    let content = parse_content(raw_line.trim_start());

    // 返回 `Line` 数据对象
    Line {
        content,
        indent,
    }
}
