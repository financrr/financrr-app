pub(crate) trait StringExt {
    fn remove_whitespaces(&self) -> String;
}

impl StringExt for String {
    fn remove_whitespaces(&self) -> String {
        self.split_whitespace().collect()
    }
}
