mod atom;
mod link;
mod domain;
mod reference;

pub trait Content<'lc> {
    fn check(&self) -> impl Content<'lc>;
}
