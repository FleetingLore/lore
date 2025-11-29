use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused)]
pub struct Node<'f> {
    content: crate::lir::content::Content<'f>,
    domain: Option<Rc<RefCell<Node<'f>>>>,
}