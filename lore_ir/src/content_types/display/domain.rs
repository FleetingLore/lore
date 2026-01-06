use std::fmt::{Display, Formatter};
use crate::content_types::data::domain::DomainContent;

#[cfg(feature = "domain")]
impl<'lc> Display for DomainContent<'lc> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
