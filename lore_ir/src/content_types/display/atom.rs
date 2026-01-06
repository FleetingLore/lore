use std::fmt::{Display, Formatter};
use crate::content_types::data::atom::AtomContent;

#[cfg(feature = "atom")]
impl Display for AtomContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
