use crate::content_types::data::Content;
use crate::content_types::data::link::LinkContent;

#[cfg(feature = "link")]
impl<'lc> Content for LinkContent<'lc> {}
