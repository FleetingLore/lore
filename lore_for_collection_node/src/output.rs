// src/output.rs

use std::fs;
use std::path::Path;
use crate::line::{Content, Line};

pub fn output_html(title: &str, lines: &Vec<Line>, path: &Path) {
    let mut html = String::new();

    html.push_str(
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{}</title>
  <link rel="stylesheet" href="https://fleetinglore.github.io/collection/collection.css">
</head>
<body>
"#,
            title,
    )
            .as_str()
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

fn line_to_html(line: &Line) -> String {
    let margin_left = line.indent * 20;

    match &line.content {
        Content::Atom(atom) => {
            format!(
                r#"<p style="margin-left: {}px">{}</p>"#,
                margin_left,
                atom
            )
        },
        Content::Link(key, value) => {
            format!(
                r#"<p style="margin-left: {}px"><a href="{}" target="_blank">{}</a></p>"#,
                margin_left,
                value,
                key
            )
        },
        Content::Domain(domain) => {
            format!(
                r#"<p style="margin-left: {}px"><strong>+ {}</strong></p>"#,
                margin_left,
                domain
            )
        }
    }
}
