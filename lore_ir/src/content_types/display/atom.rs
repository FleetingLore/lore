use std::fmt::{write, Display, Formatter};
use crate::content_types::data::atom::AtomContent;

impl<'lc> Display for AtomContent<'lc> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.meta)
    }
}
