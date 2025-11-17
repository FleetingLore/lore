//! LIR 文档根解析模块
//!
//! 这个模块负责解析完整的 LIR (Lore Intermediate Representation) 文档，
//! 将多行文本转换为结构化的层次表示。
//!
//! ## 核心功能
//!
//! - **多行解析**: 将多行文本解析为行的序列
//! - **层次结构**: 通过缩进级别隐式表示文档的层次关系
//! - **完整文档**: 表示整个 LIR 文档的根结构
//!
//! ## 处理流程
//!
//! 1. **行分割**: 将输入文本分割为单独的行
//! 2. **逐行解析**: 对每行调用 [`crate::lir::line::parse_line`]
//! 3. **结构构建**: 收集所有解析后的行，构建完整的文档结构
//!
//! ## 示例
//!
//! ```text
//! 输入:
//! root
//!   + domain
//!     nested
//!   another
//!
//! 输出:
//! Root {
//!   data: [
//!     Line { indent: 0, content: Element("root") },
//!     Line { indent: 1, content: Domain("domain") },
//!     Line { indent: 2, content: Element("nested") },
//!     Line { indent: 1, content: Element("another") },
//!   ]
//! }
//! ```
//!
//! ## 主要组件
//!
//! - [`Root`] 结构: 表示完整的 LIR 文档
//! - [`parse_root`] 函数: 从多行文本解析完整文档
//! - `Display` 实现: 用于将文档转换回文本格式

use super::line::parse_line;
use super::line::Line;

/// 表示完整的 LIR 文档，包含所有行的序列信息。
///
/// `Root` 是 LIR 解析的最终结果，它包含了文档中所有行的结构化信息。
/// 通过行的缩进级别，可以推断出文档的层次结构关系。
///
/// # 字段
///
/// - `data`: 行的向量，按文档中的顺序存储
///
/// # 层次结构
///
/// LIR 的层次结构通过缩进级别隐式表示：
/// - 相同缩进级别的行属于同一层级
/// - 更高缩进级别的行是前一个较低缩进级别行的子项
/// - 缩进级别减少表示返回到父层级
///
/// # 示例
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
/// # 参数
///
/// - `raw_lines`: 字符串切片的向量，每个元素代表文档的一行
///
/// # 返回值
///
/// 返回 [`Root`] 结构，包含解析后的所有行信息。
///
/// # 处理流程
///
/// 1. 对输入中的每一行调用 [`crate::lir::line::parse_line`]
/// 2. 收集所有解析后的行到向量中
/// 3. 返回包含这些行的 [`Root`] 结构
///
/// # 注意
///
/// - 空行会被保留并解析为内容为空的元素
/// - 行的顺序和缩进级别完全保留
/// - 不进行层次结构的验证（如缩进级别是否合理）
///
/// # 示例
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
/// # 示例
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