pub struct Directive<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> Directive<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Directive<'a> {
        Directive { key, value }
    }

    pub fn get_key_lowercase(&self) -> String {
        self.key.to_lowercase()
    }

    pub fn get_value(&self) -> &str {
        self.value
    }
}
