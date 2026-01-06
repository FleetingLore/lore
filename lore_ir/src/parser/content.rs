use crate::data::content::Content;

pub fn parse_content(content: &str) -> Content<'_> {
    if content.is_empty() {
        Content::Nothing
    } else {
        Content::Element(content)
    }
}
