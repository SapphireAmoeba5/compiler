#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<String>,
    pub return_count: usize,
    pub assembly: String,
}

impl Function {
    pub fn new(name: &str, arguments: Vec<String>, return_count: usize) -> Self {
        Self {
            name: name.to_string(),
            arguments: arguments,
            return_count: return_count,
            assembly: String::new(),
        }
    }
}
