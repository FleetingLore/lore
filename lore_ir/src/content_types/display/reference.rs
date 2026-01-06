use std::fmt::{Display, Formatter};
use crate::content_types::data::reference::ReferenceContent;

#[cfg(feature = "reference")]
impl<'lc> Display for ReferenceContent<'lc> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
