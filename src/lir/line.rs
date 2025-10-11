use super::content::Content;
use super::content::parse_content;

/// 每一行的信息包括其缩进和数据
pub struct Line<'f> {
    pub indent: usize,
    pub content: Content<'f>,
}

/// 为了方便测试，为 `Line` 实现 `Display`。
impl<'f> std::fmt::Display for Line<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "  ".repeat(self.indent), self.content)
    }
}

/// 借助 `take_while()` 和 `parse_content()` 简洁地实现单行解析。
pub fn parse_line(raw_line: &str) -> Line<'_> {
    let indent = raw_line
        .chars()
        .take_while(|&c| c == ' ')
        .count() / 2;
    let content = parse_content(raw_line.trim_start());

    Line {
        content,
        indent,
    }
}

/// 集成测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_no_indent_element() {
        let line = parse_line("simple element");
        assert!(matches!(line.content, Content::Element("simple element")));
        assert_eq!(line.indent, 0);
    }

    #[test]
    fn test_parse_content_domain() {
        let content = parse_content("+ domain text");
        assert!(matches!(content, Content::Domain("domain text")));
    }

    #[test]
    fn test_parse_content_element_with_plus_no_space() {
        let content = parse_content("+no space");
        assert!(matches!(content, Content::Element("+no space")));
    }

    #[test]
    fn test_parse_content_empty() {
        let content = parse_content("");
        assert!(matches!(content, Content::Element("")));
    }

    #[test]
    fn test_parse_line_no_indent_domain() {
        let line = parse_line("+ domain lir");
        assert!(matches!(line.content, Content::Domain("domain lir")));
        assert_eq!(line.indent, 0);
    }

    #[test]
    fn test_parse_line_with_indent_element() {
        let line = parse_line("  indented element");
        assert!(matches!(line.content, Content::Element("indented element")));
        assert_eq!(line.indent, 1);
    }

    #[test]
    fn test_parse_line_with_indent_domain() {
        let line = parse_line("    + indented domain");
        assert!(matches!(line.content, Content::Domain("indented domain")));
        assert_eq!(line.indent, 2); // 4 spaces = 2 indent levels
    }

    #[test]
    fn test_parse_line_only_spaces() {
        let line = parse_line("    ");
        assert!(matches!(line.content, Content::Element("")));
        assert_eq!(line.indent, 2);
    }

    #[test]
    fn test_display_line_element_no_indent() {
        let line = Line {
            content: Content::Element("element"),
            indent: 0,
        };
        assert_eq!(format!("{}", line), "element");
    }

    #[test]
    fn test_display_line_element_with_indent() {
        let line = Line {
            content: Content::Element("element"),
            indent: 2,
        };
        assert_eq!(format!("{}", line), "    element");
    }

    #[test]
    fn test_display_line_domain_no_indent() {
        let line = Line {
            content: Content::Domain("domain"),
            indent: 0,
        };
        assert_eq!(format!("{}", line), "+ domain");
    }

    #[test]
    fn test_display_line_domain_with_indent() {
        let line = Line {
            content: Content::Domain("domain"),
            indent: 1,
        };
        assert_eq!(format!("{}", line), "  + domain");
    }

    #[test]
    fn test_integration_parse_and_display() {
        let test_cases = vec![
            ("no indent", "no indent"),
            ("  two space indent", "  two space indent"),
            ("+ domain", "+ domain"),
            ("  + indented domain", "  + indented domain"),
            ("    four space element", "    four space element"),
        ];

        for (input, expected) in test_cases {
            let line = parse_line(input);
            assert_eq!(format!("{}", line), expected);
        }
    }

    #[test]
    fn test_odd_number_spaces_rounds_down() {
        let line = parse_line("     odd spaces");
        assert_eq!(line.indent, 2);
    }
}
