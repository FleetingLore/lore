use std::fmt::{Display, Formatter};
use crate::content_types::data::link::LinkContent;

#[cfg(feature = "link")]
impl<'lc> Display for LinkContent<'lc> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", &self.meta, &self.value)
    }
}
