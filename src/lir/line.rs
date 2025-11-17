//! LIR 行解析模块
//!
//! 这个模块负责解析 LIR (Lore Intermediate Representation) 中的完整行，
//! 包括缩进级别和内容类型的组合解析。
//!
//! ## 核心功能
//!
//! - **缩进解析**: 计算每行的缩进级别（每 2 个空格为一级）
//! - **内容组合**: 将缩进信息与内容类型组合成完整的行信息
//! - **格式化输出**: 提供可读的行显示格式
//!
//! ## 解析规则
//!
//! 1. **缩进计算**: 前导空格数量除以 2 得到缩进级别
//! 2. **内容传递**: 去除前导空格后的内容传递给 [`crate::lir::content::parse_content`]
//! 3. **组合结果**: 缩进级别和内容类型组合为 [`Line`] 结构
//!
//! ## 示例
//!
//! ```text
//! "simple"          -> indent=0, Content::Element("simple")
//! "  indented"      -> indent=1, Content::Element("indented")  
//! "+ domain"        -> indent=0, Content::Domain("domain")
//! "  + domain"      -> indent=1, Content::Domain("domain")
//! "    deep"        -> indent=2, Content::Element("deep")
//! ```
//!
//! ## 主要组件
//!
//! - [`Line`] 结构: 包含缩进级别和内容类型的完整行信息
//! - [`parse_line`] 函数: 从原始字符串解析出行信息
//! - `Display` 实现: 用于格式化输出，重现原始格式

use super::content::Content;
use super::content::parse_content;

/// 表示 LIR 中的完整行信息，包含缩进级别和内容。
///
/// 每行 LIR 文档都由两个主要部分组成：
/// - 缩进级别：表示在层次结构中的嵌套深度
/// - 内容类型：决定该行的语义含义
///
/// # 字段
///
/// - `indent`: 缩进级别，0 表示根级别，每增加 1 表示一级嵌套
/// - `content`: 行内容类型，由 [`Content`] 枚举表示
///
/// # 示例
///
/// ```
/// use crate::lir::{Line, Content};
///
/// let line = Line {
///     indent: 1,
///     content: Content::Element("indented element"),
/// };
/// assert_eq!(format!("{}", line), "  indented element");
/// ```
pub struct Line<'f> {
    /// 缩进级别，表示行的嵌套深度
    ///
    /// - 0: 根级别，无缩进
    /// - 1: 一级缩进（2 个空格）
    /// - 2: 二级缩进（4 个空格）
    /// - 以此类推...
    pub indent: usize,

    /// 行的内容类型，决定该行的语义含义
    pub content: Content<'f>,
}

/// 为了方便测试，为 `Line` 实现 `Display`。
///
/// 这个实现能够将 `Line` 结构转换回其原始的文本表示形式，
/// 包括正确的缩进和内容格式。
///
/// # 示例
///
/// ```
/// use crate::lir::{Line, Content};
///
/// let line = Line {
///     indent: 2,
///     content: Content::Domain("example"),
/// };
/// assert_eq!(format!("{}", line), "    + example");
/// ```
impl<'f> std::fmt::Display for Line<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "  ".repeat(self.indent), self.content)
    }
}

/// 解析单行 LIR 文本，提取缩进信息和内容类型。
///
/// 这是 LIR 解析流水线中的关键步骤，它将原始字符串转换为结构化的行信息。
///
/// # 参数
///
/// - `raw_line`: 原始行字符串，可能包含前导空格
///
/// # 返回值
///
/// 返回 [`Line`] 结构，包含解析出的缩进级别和内容类型。
///
/// # 处理流程
///
/// 1. **计算缩进**: 统计前导空格数量，每 2 个空格为一级缩进
/// 2. **提取内容**: 去除前导空格，得到纯内容字符串
/// 3. **解析内容**: 调用 [`crate::lir::content::parse_content`] 解析内容类型
/// 4. **组合结果**: 返回包含缩进和内容的 [`Line`] 结构
///
/// # 注意
///
/// - 奇数个前导空格会被向下取整（3 个空格 = 1 级缩进）
/// - 空行和纯空格行会被解析为 0 内容的元素
///
/// # 示例
///
/// ```
/// use crate::lir::{parse_line, Content};
///
/// let line = parse_line("  + domain");
/// assert_eq!(line.indent, 1);
/// assert!(matches!(line.content, Content::Domain("domain")));
/// ```
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