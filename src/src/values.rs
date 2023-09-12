pub struct Number {
    pub value: f64
}

impl Number {
    pub fn __repr__(&self) -> String {
        return self.value.to_string()
    }
}
