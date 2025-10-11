/// 根据是否伴随子域，划分出两种行类型。
pub enum Content<'f> {
    Element(&'f str),
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

/// 借助 str 的 strip_prefix() 方法，简洁地实现格式识别。
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
