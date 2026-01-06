use crate::content_types::data::Content;
use crate::content_types::data::domain::DomainContent;

#[cfg(feature = "domain")]
impl<'lc> Content for DomainContent<'lc> {

}