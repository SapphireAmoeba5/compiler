pub struct Identifier {
    pub name: String,
    // The offset from the rbx register
    pub offset: usize,
}

impl Identifier {
    pub fn new(name: &str, offset: usize) -> Self {
        Self {
            name: name.to_string(),
            offset: offset,
        }
    }
}
