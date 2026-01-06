use crate::data::line::Line;

impl<'f> std::fmt::Display for Line<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", "  ".repeat(self.indent), self.content)
    }
}
