use crate::structure::data::root::Root;

impl<'f> std::fmt::Display for Root<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.indented_lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
