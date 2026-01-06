impl<'f> std::fmt::Display for Root<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.data {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
