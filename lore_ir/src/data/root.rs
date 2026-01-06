use crate::data::line::Line;

pub struct Root<'f> {
    pub indented_lines: Vec<Line<'f>>,
}
