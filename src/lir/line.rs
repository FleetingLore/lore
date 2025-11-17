//! ## 行模块
//!
//! 计算每行的缩进级别，每 `2` 个空格为一级。
//!
//! ```text
//! "[ * ]"          -> indent=0, Content::Element("[ * ]")
//! "  [ * ]"        -> indent=1, Content::Element("[ * ]")  
//! "+ [ * ]"        -> indent=0, Content::Domain("[ * ]")
//! "  + [ * ]"      -> indent=1, Content::Domain("[ * ]")
//! "    [ * ]"      -> indent=2, Content::Element("[ * ]")
//! ```

use super::content::Content;
use super::content::parse_content;

/// 行的信息，包含缩进和内容。
///
/// 缩进，嵌套关系判定的关键依据。
/// 内容，决定该行的自身信息。
///
/// ```
/// use crate::lir::{Line, Content};
///
/// let line = Line {
///     indent: 1,
///     content: Content::Element("[ * indented element ]"),
/// };
/// assert_eq!(format!("{}", line), "  [ * indented element ]");
/// ```
pub struct Line<'f> {
    /// 缩进
    pub indent: usize,

    /// 内容
    pub content: Content<'f>,
}

/// 为了方便测试，为 `Line` 实现 `Display`，转换为原始文本。
///
/// ```
/// use crate::lir::{Line, Content};
///
/// let line = Line {
///     indent: 2,
///     content: Content::Domain("[ * ]"),
/// };
/// assert_eq!(format!("{}", line), "    + [ * ]");
/// ```
impl<'f> std::fmt::Display for Line<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "  ".repeat(self.indent), self.content)
    }
}

/// 解析单行字符串，返回一个 `Line` 数据对象。
///
/// 奇数个前导空格会被向下取整。
/// 空行和纯空格行会被解析为 0 内容的元素。
///
/// ```
/// use crate::lir::{parse_line, Content};
///
/// let line = parse_line("  + [ * ]");
/// assert_eq!(line.indent, 1);
/// assert!(matches!(line.content, Content::Domain("[ * ]")));
/// ```
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

// 测试模块如此。测试结果与预期一致。
#[cfg(test)]
mod tests {
    use super::*;
    
    // 一般的要素
    #[test]
    fn test_parse_line_no_indent_element() {
        let line = parse_line("simple element");
        assert!(matches!(line.content, Content::Element("simple element")));
        assert_eq!(line.indent, 0);
    }

    // 一般的领域
    #[test]
    fn test_parse_content_domain() {
        let content = parse_content("+ domain text");
        assert!(matches!(content, Content::Domain("domain text")));
    }
    

    // 空要素
    #[test]
    fn test_parse_content_empty() {
        let content = parse_content("");
        assert!(matches!(content, Content::Element("")));
    }

    // 空领域
    #[test]
    fn test_parse_line_no_indent_domain() {
        let line = parse_line("+ domain lir");
        assert!(matches!(line.content, Content::Domain("domain lir")));
        assert_eq!(line.indent, 0);
    }

    // `+` 后未打空格则识别为要素
    #[test]
    fn test_parse_content_element_with_plus_no_space() {
        let content = parse_content("+no space");
        assert!(matches!(content, Content::Element("+no space")));
    }

    // 有所进的要素
    #[test]
    fn test_parse_line_with_indent_element() {
        let line = parse_line("  indented element");
        assert!(matches!(line.content, Content::Element("indented element")));
        assert_eq!(line.indent, 1);
    }

    // 有缩进的领域
    #[test]
    fn test_parse_line_with_indent_domain() {
        let line = parse_line("    + indented domain");
        assert!(matches!(line.content, Content::Domain("indented domain")));
        assert_eq!(line.indent, 2); // 4 spaces = 2 indent levels
    }

    // 空格构成的行
    #[test]
    fn test_parse_line_only_spaces() {
        let line = parse_line("    ");
        assert!(matches!(line.content, Content::Element("")));
        assert_eq!(line.indent, 2);
    }

    // 无缩进的要素行的演示
    #[test]
    fn test_display_line_element_no_indent() {
        let line = Line {
            content: Content::Element("element"),
            indent: 0,
        };
        assert_eq!(format!("{}", line), "element");
    }

    // 有缩进的要素行的演示
    #[test]
    fn test_display_line_element_with_indent() {
        let line = Line {
            content: Content::Element("element"),
            indent: 2,
        };
        assert_eq!(format!("{}", line), "    element");
    }

    // 无缩进的领域行的演示
    #[test]
    fn test_display_line_domain_no_indent() {
        let line = Line {
            content: Content::Domain("domain"),
            indent: 0,
        };
        assert_eq!(format!("{}", line), "+ domain");
    }

    // 有缩进的领域行的演示
    #[test]
    fn test_display_line_domain_with_indent() {
        let line = Line {
            content: Content::Domain("domain"),
            indent: 1,
        };
        assert_eq!(format!("{}", line), "  + domain");
    }

    // 奇数缩进的判定，向下取值
    #[test]
    fn test_odd_number_spaces_rounds_down() {
        let line = parse_line("     odd spaces");
        assert_eq!(line.indent, 2);
    }
}