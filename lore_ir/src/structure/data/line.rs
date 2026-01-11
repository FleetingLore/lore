use crate::content_types::as_content::Content;

pub struct Line<'f> {
    pub indent: usize,
    pub content: Option<Box<dyn Content<'f>>>,
}
