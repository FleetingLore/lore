use crate::content_types::as_content::Content;
use crate::content_types::data::domain::DomainContent;

#[cfg(feature = "domain")]
impl<'lc, 'f> Content<'f> for DomainContent<'lc> {
    fn check(&self) -> dyn Content<'f> {
        todo!()
    }
}