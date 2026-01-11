use crate::line::{Content, Line};

pub fn parse_line(line: &str) -> Line {
    let trimmed = line.trim_start();
    let indent = (line.len() - trimmed.len()) / 2;

    if trimmed.starts_with('+') && trimmed.len() > 1 {
        let content = Content::Domain(trimmed[1..].trim().to_string());
        Line {
            indent,
            content
        }
    } else {
        if let Some(pos) = trimmed.find('=') {
            let before_eq = trimmed[..pos].trim();
            let after_eq = trimmed[pos + 1..].trim();

            let content = Content::Link(before_eq.to_string(), after_eq.to_string());
            Line {
                indent,
                content
            }
        } else {
            let content = Content::Atom(trimmed.to_string());
            Line {
                indent,
                content
            }
        }
    }
}