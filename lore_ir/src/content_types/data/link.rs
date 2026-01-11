#[cfg(feature = "link")]
pub(crate) struct LinkContent<'lc> {
    pub(crate) meta: &'lc str,
    pub(crate) value: &'lc str,
}
