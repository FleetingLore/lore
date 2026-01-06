use crate::content_types::data::Content;

#[cfg(feature = "domain")]
struct DomainContent {}

#[cfg(feature = "domain")]
impl Content for DomainContent {
    
}
