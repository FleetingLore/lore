use crate::data::content::Content;

pub struct Line<'f> {
    pub indent: usize,
    pub content: Content<'f>,
}
