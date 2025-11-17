//! LIR 内容解析模块
//!
//! 这个模块负责解析 LIR (Lore Intermediate Representation) 中的单行内容，
//! 将原始文本区分为两种类型：普通元素和域定义。
//!
//! ## 核心概念
//!
//! LIR 使用简单的语法规则来区分内容类型：
//!
//! - **普通元素 (Element)**: 普通的文本行，不包含特殊前缀
//! - **域定义 (Domain)**: 以 "+ " 开头的行，表示一个可以包含子元素的域
//!
//! ## 解析规则
//!
//! 1. 如果一行以 "+ " 开头（加号后必须有空格），则解析为 `Content::Domain`
//! 2. 所有其他情况都解析为 `Content::Element`
//! 3. 空字符串解析为空的 `Content::Element`
//!
//! ## 示例
//!
//! ```text
//! 普通文本          -> Element("普通文本")
//! + 域定义          -> Domain("域定义")
//! +no space        -> Element("+no space")  // 注意：缺少空格
//! +                -> Element("+")          // 注意：缺少空格和内容
//! +                -> Domain("")            // 注意：有空格但无内容
//! ```
//!
//! ## 主要组件
//!
//! - [`Content`] 枚举: 表示两种内容类型
//! - [`parse_content`] 函数: 从字符串解析内容类型
//! - `Display` 实现: 用于格式化输出
//!
//! 这个模块与 [`crate::lir::line`] 和 [`crate::lir::root`] 模块协同工作，共同完成 LIR 文档的完整解析。

/// 表示 LIR 中的单行内容类型。
///
/// 根据 LIR 语法规范，内容被划分为两种基本类型：
/// - 普通元素：包含任意文本内容
/// - 域定义：以 "+ " 前缀标识，可以包含子元素
///
/// # 生命周期
///
/// 使用生命周期 `'f` 来借用原始字符串切片，避免不必要的内存分配。
///
/// # 示例
///
/// ```
/// use crate::lir::content::{Content, parse_content};
///
/// let element = parse_content("普通文本");
/// assert!(matches!(element, Content::Element("普通文本")));
///
/// let domain = parse_content("+ 域定义");
/// assert!(matches!(domain, Content::Domain("域定义")));
/// ```
pub enum Content<'f> {
    /// 普通元素，包含任意文本内容
    ///
    /// 普通元素是 LIR 中的基本构建块，可以出现在任何缩进级别。
    /// 它们不包含特殊的语法含义，只是纯文本内容。
    Element(&'f str),

    /// 域定义，以 "+ " 前缀标识
    ///
    /// 域是一种特殊的内容类型，它可以包含子元素（普通元素或其他域）。
    /// 域为 LIR 文档提供了层次结构的能力。
    Domain(&'f str)
}

/// 为了便于测试，这里给 `Content<'f>` 实现 `Display`。
impl<'f> std::fmt::Display for Content<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Content::Element(tokens) => write!(f, "{}", tokens),
            Content::Domain(tokens) => write!(f, "+ {}", tokens)
        }
    }
}

/// 解析字符串内容并返回相应的 `Content` 类型。
///
/// 这个函数是内容解析的核心，它根据简单的前缀规则判断内容类型。
///
/// # 参数
///
/// - `content`: 要解析的字符串切片，应该是不包含前导空格的纯内容
///
/// # 返回值
///
/// 返回 `Content` 枚举，表示解析出的内容类型。
///
/// # 规则
///
/// 1. 如果内容以 "+ " 开头 → `Content::Domain(剩余内容)`
/// 2. 其他所有情况 → `Content::Element(原内容)`
///
/// # 注意
///
/// 这个函数期望接收已经去除前导空格的内容。在实际使用中，
/// 通常先由 [`crate::lir::line::parse_line`] 处理缩进，然后将剩余内容传递给此函数。
///
/// # 示例
///
/// ```
/// use crate::lir::content::{parse_content, Content};
///
/// assert!(matches!(parse_content("hello"), Content::Element("hello")));
/// assert!(matches!(parse_content("+ world"), Content::Domain("world")));
/// assert!(matches!(parse_content("+no space"), Content::Element("+no space")));
/// ```
pub fn parse_content(content: &str) -> Content<'_> {
    if let Some(rest) = content.strip_prefix("+ ") {
        Content::Domain(rest)
    } else {
        Content::Element(content)
    }
}

/// 测试模块如此。测试结果与预期一致。
#[cfg(test)]
mod tests {
    use super::*;

    // 现有测试保持不变...
    #[test]
    fn test_element_content() {
        let content = parse_content("hello world");
        match content {
            Content::Element(text) => assert_eq!(text, "hello world"),
            _ => panic!("Expected Element variant"),
        }
    }

    #[test]
    fn test_domain_content() {
        let content = parse_content("+ domain content");
        match content {
            Content::Domain(text) => assert_eq!(text, "domain content"),
            _ => panic!("Expected Domain variant"),
        }
    }

    #[test]
    fn test_domain_content_but_no_space_after_plus() {
        let content = parse_content("+no space");
        match content {
            Content::Element(text) => assert_eq!(text, "+no space"),
            _ => panic!("Expected Element variant"),
        }
    }

    #[test]
    fn test_empty_string() {
        let content = parse_content("");
        match content {
            Content::Element(text) => assert_eq!(text, ""),
            _ => panic!("Expected Element variant"),
        }
    }

    #[test]
    fn test_only_plus_and_a_space() {
        let content = parse_content("+ ");
        match content {
            Content::Domain(text) => assert_eq!(text, ""),
            _ => panic!("Expected Domain variant"),
        }
    }

    #[test]
    fn test_display_implementation() {
        let element = Content::Element("test element");
        let domain = Content::Domain("test domain");

        assert_eq!(format!("{}", element), "test element");
        assert_eq!(format!("{}", domain), "+ test domain");
    }

    #[test]
    fn test_integration_parse_and_display() {
        let test_cases = vec![
            ("normal text", "normal text"),
            ("+ domain text", "+ domain text"),
        ];

        for (input, expected) in test_cases {
            let content = parse_content(input);
            assert_eq!(format!("{}", content), expected);
        }
    }
}