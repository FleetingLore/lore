use super::line::parse_line;
use super::line::Line;

pub struct Root<'f> {
    pub data: Vec<Line<'f>>,
}

pub fn parse_root(raw_lines: Vec<&str>) -> Root<'_> {
    let data = raw_lines
        .into_iter()
        .map(parse_line)
        .collect();

    Root { data }
}

// 为 `Root` 实现 `Display` 以便测试
impl<'f> std::fmt::Display for Root<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.data {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::content::Content;
    use super::*;

    #[test]
    fn test_parse_root_empty() {
        let root = parse_root(vec![]);
        assert_eq!(root.data.len(), 0);
    }

    #[test]
    fn test_parse_root_single_line() {
        let root = parse_root(vec!["simple lir"]);
        assert_eq!(root.data.len(), 1);

        if let Content::Element(text) = &root.data[0].content {
            assert_eq!(*text, "simple lir");
        } else {
            panic!("Expected Element variant");
        }
        assert_eq!(root.data[0].indent, 0);
    }

    #[test]
    fn test_parse_root_multiple_lines() {
        let lines = vec![
            "root element",
            "  indented element",
            "  + domain lir",
            "    deeply nested",
        ];

        let root = parse_root(lines);
        assert_eq!(root.data.len(), 4);

        // 检查第一行
        assert!(matches!(root.data[0].content, Content::Element("root element")));
        assert_eq!(root.data[0].indent, 0);

        // 检查第二行
        assert!(matches!(root.data[1].content, Content::Element("indented element")));
        assert_eq!(root.data[1].indent, 1);

        // 检查第三行
        assert!(matches!(root.data[2].content, Content::Domain("domain lir")));
        assert_eq!(root.data[2].indent, 1);

        // 检查第四行
        assert!(matches!(root.data[3].content, Content::Element("deeply nested")));
        assert_eq!(root.data[3].indent, 2);
    }

    #[test]
    fn test_parse_root_mixed_content() {
        let lines = vec![
            "+ domain at root",
            "normal element",
            "  + indented domain",
            "    normal indented",
        ];

        let root = parse_root(lines);
        assert_eq!(root.data.len(), 4);

        assert!(matches!(root.data[0].content, Content::Domain("domain at root")));
        assert!(matches!(root.data[1].content, Content::Element("normal element")));
        assert!(matches!(root.data[2].content, Content::Domain("indented domain")));
        assert!(matches!(root.data[3].content, Content::Element("normal indented")));
    }

    #[test]
    fn test_parse_root_with_empty_lines() {
        let lines = vec![
            "first lir",
            "",
            "  indented lir",
            "    ",
        ];

        let root = parse_root(lines);
        assert_eq!(root.data.len(), 4);

        // 空行应该被解析为 Element("")
        assert!(matches!(root.data[1].content, Content::Element("")));
        assert_eq!(root.data[1].indent, 0);

        // 只有空格的行
        assert!(matches!(root.data[3].content, Content::Element("")));
        assert_eq!(root.data[3].indent, 2);
    }

    #[test]
    fn test_root_display() {
        let lines = vec![
            "root",
            "  + domain",
            "    nested",
        ];

        let root = parse_root(lines);
        let output = format!("{}", root);
        let expected = "root\n  + domain\n    nested\n";

        assert_eq!(output, expected);
    }
}
