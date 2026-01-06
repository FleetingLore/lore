use crate::content_types::data::Content;

#[cfg(feature = "link")]
pub(crate) struct LinkContent<'lc> {
    meta: &'lc str,
    value: &'lc str,
}
