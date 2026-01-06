use crate::content_types::data::Content;

#[cfg(feature = "link")]
struct LinkContent {}

#[cfg(feature = "link")]
impl Content for LinkContent {}
