pub(crate) trait SplitEmptyLines {
    fn split_empty_lines(&self) -> impl Iterator<Item = &str>;
}

impl SplitEmptyLines for str {
    fn split_empty_lines(&self) -> impl Iterator<Item = &str> {
        self.split("\r\n\r\n").flat_map(|s: &str| s.split("\n\n"))
    }
}
