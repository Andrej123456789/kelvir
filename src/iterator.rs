pub struct Iterator {
    text: String,
    id: usize,
    pub first_run: bool
}

impl Iterator {
    pub fn new(_text: String) -> Self {
        Iterator {
            text: _text,
            id: 0,
            first_run: true,
        }
    }

    pub fn first(&self) -> char {
        return self.text.chars().nth(0).unwrap();
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
