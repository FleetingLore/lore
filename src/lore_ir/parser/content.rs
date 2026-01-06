pub fn parse_content(content: &str) -> Content<'_> {
    if content.is_empty() {
        return Content::None();
    }

    if let Some(rest) = content.strip_prefix("+ ") {
        Content::Domain(rest)
    } else {
        Content::Element(content)
    }
}
