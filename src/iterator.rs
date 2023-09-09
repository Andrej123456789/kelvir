pub struct Iterator {
    text: String,
    id: usize,
}

impl Iterator {
    pub fn new(_text: String) -> Self {
        Iterator {
            text: _text,
            id: 0,
        }
    }

    pub fn next(&mut self) -> char {
        self.id += 1;
        
        if let Some(ch) = self.text.chars().nth(self.id) {
            return ch
        } else {
            return '\0'
        }
    }
}
