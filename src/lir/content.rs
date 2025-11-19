//! ## 内容模块
//!
//! 每一行的数据，包括缩进级别和内容。
//! 根据是否伴随子域，内容分为要素或领域这两种类型。
//!
//! 以 `+ ` 开头，加号后有空格，则解析为 `Content::Domain`。
//! 其余其他情况，解析为 `Content::Element`。
//!
//! ```text
//! [ * content ]            -> Element("[ * content ]")
//! + [ * content ]          -> Domain("+ [ * content ]")
//! +[ * content ]           -> Element("+[ * content ]")     // `+` 后无空格。
//! +                        -> Element("+")                  // `+` 后无空格和内容。
//! +                        -> Domain("")                    // `+` 后有空格但无内容。
//! ```

/// 表示 LIR 中的单行内容类型。
///
/// 使用生命周期 `'f` 来借用原始字符串切片
///
/// ```
/// use crate::lir::content::{Content, parse_content};
///
/// let element = parse_content("[ * ]");
/// assert!(matches!(element, Content::Element("[ * ]")));
///
/// let domain = parse_content("+ [ * ]");
/// assert!(matches!(domain, Content::Domain("[ * ]")));
/// ```
pub enum Content<'f> {
    /// 要素，无特殊语法
    Element(&'f str),

    /// 领域，以 "+ " 前缀标识
    Domain(&'f str)
}

/// 为了便于测试，为 `Content<'f>` 实现 `Display`。
impl<'f> std::fmt::Display for Content<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Content::Element(tokens) => write!(f, "{}", tokens),
            Content::Domain(tokens) => write!(f, "+ {}", tokens)
        }
    }
}

/// 解析字符串，返回一个 `Content` 数据对象。
///
/// 对于 `"+ [ * ]"`，返回 `Content::Domain("[ * ]")`。
/// 其他所有情况，即对于 `"[ * ]"`，返回 `Content::Element("[ * ]")`。
///
/// ```
/// use crate::lir::content::{parse_content, Content};
///
/// assert!(matches!(parse_content("[ * ]"), Content::Element("[ * ]")));
/// assert!(matches!(parse_content("+ [ * ]"), Content::Domain("[ * ]")));
/// assert!(matches!(parse_content("+[ *content ]"), Content::Element("+[ * content ]")));
/// ```
///
/// 实际上，这个函数期望接收已经去除前导空格的内容。
/// 即先由 `lore::lir::line::parse_line` 计算缩进，然后将剩余部分作为内容，传递给此函数。
///
pub fn parse_content(content: &str) -> Content<'_> {
    if let Some(rest) = content.strip_prefix("+ ") {
        Content::Domain(rest)
    } else {
        Content::Element(content)
    }
}

// 测试模块如此。测试结果与预期一致。
#[cfg(test)]
mod tests {
    use super::*;

    // 一般的要素
    #[test]
    fn test_element_content() {
        let content = parse_content("hello world");
        match content {
            Content::Element(text) => assert_eq!(text, "hello world"),
            _ => panic!("Expected Element variant"),
        }
    }

    // 一般的领域
    #[test]
    fn test_domain_content() {
        let content = parse_content("+ domain content");
        match content {
            Content::Domain(text) => assert_eq!(text, "domain content"),
            _ => panic!("Expected Domain variant"),
        }
    }

    // 空要素
    #[test]
    fn test_empty_string() {
        let content = parse_content("");
        match content {
            Content::Element(text) => assert_eq!(text, ""),
            _ => panic!("Expected Element variant"),
        }
    }

    // 空领域
    #[test]
    fn test_only_plus_and_a_space() {
        let content = parse_content("+ ");
        match content {
            Content::Domain(text) => assert_eq!(text, ""),
            _ => panic!("Expected Domain variant"),
        }
    }

    // `+` 后未打空格则识别为要素
    #[test]
    fn test_domain_content_but_no_space_after_plus() {
        let content = parse_content("+no space");
        match content {
            Content::Element(text) => assert_eq!(text, "+no space"),
            _ => panic!("Expected Element variant"),
        }
    }

    // 测试 `Display` 在 `Content` 的实现
    #[test]
    fn test_display_implementation() {
        let element = Content::Element("test element");
        let domain = Content::Domain("test domain");

        assert_eq!(format!("{}", element), "test element");
        assert_eq!(format!("{}", domain), "+ test domain");
    }
}