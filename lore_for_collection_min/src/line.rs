use std::fmt::{Display, Formatter};

pub struct Line {
    pub indent: usize,
    pub content: Content,
}

pub enum Content {
    Atom(String),
    Link(String, String),
    Domain(String)
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let spaces = "  ".repeat(self.indent);
        match &self.content {
            Content::Atom(atom) => write!(f, "{}{}", spaces, atom),
            Content::Link(key, value) => write!(f, "{}{} = {}", spaces, key, value),
            Content::Domain(domain) => write!(f, "{}+ {}", spaces, domain)
        }
    }
}
