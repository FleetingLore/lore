use std::fs;
use std::path::Path;
use crate::line::{Content, Line};

// 生成 html 文件
pub fn output_html(title: &str, lines: Vec<Line>, path: &Path) {
    let mut html = String::new();

    html.push_str(
        format!(
            r##"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{}</title>
<style>
* {{
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}}

body {{
    font-family: Arial, sans-serif;
    background-color: #f8f9fa;
    color: #212529;
    min-height: 100vh;
    padding: 2rem;
    line-height: 1.6;
}}

p {{
    margin-bottom: 1rem;
}}

a {{
    text-decoration: none;
    color: #010150;
    font-size: 1.1rem;
    font-weight: 500;
    transition: all 0.2s ease;
}}

a:hover {{
    color: #0ad3b6;
}}

a:active {{
    color: #084298;
    text-decoration: underline;
}}

</style>
</head>
<body>
"##,
            title,
        ).as_str()
    );

    for line in lines {
        html.push_str(line_to_html(&line).as_str());
    }

    html.push_str(
        r#"
</body>
</html>"#
    );

    fs::write(path, html).unwrap();
}

// 以行为单位的转换
fn line_to_html(line: &Line) -> String {
    // 缩进参数
    let margin_left = line.indent * 20;

    // 构建返回标签
    match &line.content {
        // 原子
        Content::Atom(atom) => {
            format!(
                r#"<p style="margin-left: {}px">{}</p>"#,
                margin_left,
                atom
            )
        },

        // 链接
        Content::Link(key, value) => {
            format!(
                r#"<p style="margin-left: {}px"><a href="{}" target="_blank">{}</a></p>"#,
                margin_left,
                value,
                key
            )
        },

        // 领域
        Content::Domain(domain) => {
            format!(
                r#"<p style="margin-left: {}px"><strong>+ {}</strong></p>"#,
                margin_left,
                domain
            )
        }
    }
}
