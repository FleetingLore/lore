// src/line.rs

use std::fmt::{Display, Formatter};

pub struct Line<'l> {
    pub indent: usize,
    pub content: Content<'l>,
}

#[derive(Debug)]
pub enum Content<'l> {
    Atom(&'l str),
    Link(&'l str, &'l str),
    Domain(&'l str)
}

impl<'l> Display for Line<'l> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let spaces = "  ".repeat(self.indent);
        match &self.content {
            Content::Atom(atom) => write!(f, "{}{}", spaces, atom),
            Content::Link(key, value) => write!(f, "{}{} = {}", spaces, key, value),
            Content::Domain(domain) => write!(f, "{}+ {}", spaces, domain)
        }
    }
}
