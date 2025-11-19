//! ## 根模块
//!
//! 根是最高层的领域。
//!
//! ```text
//! 输入 lore 文本
//! root
//!   + domain
//!     nested
//!   another
//!
//! 输出 rust 数据对象
//! Root {
//!   data: [
//!     Line { indent: 0, content: Element("root") },
//!     Line { indent: 1, content: Domain("domain") },
//!     Line { indent: 2, content: Element("nested") },
//!     Line { indent: 1, content: Element("another") },
//!   ]
//! }
//! ```

use super::line::parse_line;
use super::line::Line;

/// 表示完整的 LIR 文档，包含所有行的序列信息。
///
/// `Root` 是 LIR 解析的最终结果，它包含了文档中所有行的结构化信息。
/// 通过行的缩进级别，可以推断出文档的层次结构关系。
///
/// ```
/// use crate::lir::parse_root;
///
/// let lines = vec!["root", "  child"];
/// let root = parse_root(lines);
/// assert_eq!(root.data.len(), 2);
/// ```
pub struct Root<'f> {
    /// 文档中所有行的序列，按原始顺序存储
    ///
    /// 每行包含其缩进级别和内容类型，共同定义了文档的
    /// 层次结构和语义含义。
    pub data: Vec<Line<'f>>,
}

/// 解析多行 LIR 文本，构建完整的文档结构。
///
/// 这是 LIR 解析器的入口函数，它接收多行文本并返回结构化的文档表示。
///
/// 返回 [`Root`] 结构，包含解析后的所有行信息。
///
/// ```
/// use crate::lir::parse_root;
///
/// let lines = vec![
///     "root element",
///     "  + domain definition",
///     "    nested element",
/// ];
/// let root = parse_root(lines);
///
/// // 可以格式化输出整个文档
/// println!("{}", root);
/// ```
pub fn parse_root(raw_lines: Vec<&str>) -> Root<'_> {
    let data = raw_lines
        .into_iter()
        .map(parse_line)
        .collect();

    Root { data }
}

/// 为 `Root` 实现 `Display` 以便测试和输出。
///
/// 这个实现能够将结构化的 LIR 文档转换回其原始的文本格式，
/// 包括所有的缩进和内容格式。
///
/// ```
/// use crate::lir::parse_root;
///
/// let lines = vec!["root", "  child"];
/// let root = parse_root(lines);
/// assert_eq!(format!("{}", root), "root\n  child\n");
/// ```
impl<'f> std::fmt::Display for Root<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.data {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

// 测试模块如此。测试结果与预期一致。
#[cfg(test)]
mod tests {
    use super::super::content::Content;
    use super::*;

    // 空根
    #[test]
    fn test_parse_root_empty() {
        let root = parse_root(vec![]);
        assert_eq!(root.data.len(), 0);
    }

    // 单行构成的根
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

    // 多行构成的根
    #[test]
    fn test_parse_root_mixed_content() {
        let lines = vec![
            "+ domain at root",
            "normal element",
            "+ another domain",
            "  normal indented",
        ];

        let root = parse_root(lines);
        assert_eq!(root.data.len(), 4);

        assert!(matches!(root.data[0].content, Content::Domain("domain at root")));
        assert!(matches!(root.data[1].content, Content::Element("normal element")));
        assert!(matches!(root.data[0].content, Content::Domain("another domain")));
        assert!(matches!(root.data[1].content, Content::Element("normal indented")));
    }
}