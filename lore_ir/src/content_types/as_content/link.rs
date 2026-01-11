use crate::content_types::as_content::Content;
use crate::content_types::data::link::LinkContent;

#[cfg(feature = "link")]
impl<'lc, 'f> Content<'f> for LinkContent<'lc> {
    fn check(&self) -> dyn Content<'f> {
        todo!()
    }
}
