use crate::content_types::data::Content;
use crate::content_types::data::reference::ReferenceContent;

#[cfg(feature = "reference")]
impl<'lc> Content for ReferenceContent<'lc> {

}