// src/parser.rs

use crate::line::{Content, Line};

pub fn parse_line(line: &str) -> Line<'_> {
    let trimmed = line.trim_start();
    let indent = (line.len() - trimmed.len()) / 2;

    if trimmed.starts_with('+') && trimmed.len() > 1 {
        let content = Content::Domain(trimmed[1..].trim());
        Line {
            indent,
            content
        }
    } else {
        if let Some(pos) = trimmed.find('=') {
            let before_eq = trimmed[..pos].trim();
            let after_eq = trimmed[pos + 1..].trim();

            let content = Content::Link(before_eq, after_eq);
            Line {
                indent,
                content
            }
        } else {
            let content = Content::Atom(trimmed);
            Line {
                indent,
                content
            }
        }
    }
}
