use crate::content_types::as_content::Content;
use crate::content_types::data::reference::ReferenceContent;

#[cfg(feature = "reference")]
impl<'lc, 'f> Content<'f> for ReferenceContent<'lc> {
    fn check(&self) -> impl Content<'f> {
        todo!()
    }
}
