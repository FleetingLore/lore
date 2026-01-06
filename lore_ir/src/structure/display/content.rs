use crate::structure::data::content::Content;

impl<'f> std::fmt::Display for Content<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Content::Element(tokens) => write!(f, "{}", tokens),
            Content::Nothing => write!(f, "")
        }
    }
}
