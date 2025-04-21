pub struct Textinput {
    // Using a vector of chars instead of String because there would be problems with utf-8
    value: Vec<char>,
    title: String,
    cursor_offset: usize
}

impl Textinput {
    pub fn new(value: String, title: String) -> Self {
        let value = Vec::from_iter(value.chars());

        Self { value, title, cursor_offset: 0 }
    }
}
