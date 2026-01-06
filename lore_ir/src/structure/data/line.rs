use crate::structure::data::content::Content;

pub struct Line<'f> {
    pub indent: usize,
    pub content: Content<'f>,
}
