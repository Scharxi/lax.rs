#[derive(Debug)]
pub struct LaxError {
    pub line: usize,
    pub message: String
}

impl LaxError {
    pub fn error(line: usize, message: String) -> LaxError {
        LaxError { line, message }
    }

    pub fn report(&self, loc: &str) {
        eprintln!("[line {}] Error{}: {}", self.line, loc, self.message)
    }
}