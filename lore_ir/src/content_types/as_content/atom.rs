use crate::content_types::as_content::Content;
use crate::content_types::data::atom::AtomContent;

impl<'lc, 'f> Content<'f> for AtomContent<'lc> {
    fn check(&'lc self) -> AtomContent<'lc> {
        todo!()
    }
}
