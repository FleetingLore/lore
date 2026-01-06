use crate::content_types::data::Content;

#[cfg(feature = "reference")]
struct ReferenceContent {}

#[cfg(feature = "reference")]
impl Content for ReferenceContent {
    
}
