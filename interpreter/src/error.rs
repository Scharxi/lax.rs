use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct LaxError {
    pub token: Option<Token>,
    pub line: usize,
    pub message: String
}

impl LaxError {
    pub fn error(line: usize, message: String) -> LaxError {
        let err = LaxError { token: None, line, message };
        err.report("");
        err
    }

    pub fn parse_error(token: Token, message: &str) -> LaxError {
        let err = LaxError { token: Some(token.clone()), line: token.clone().line, message: message.to_owned() };
        err.report("");
        err
    }

    pub fn report(&self, loc: &str) {
        if let Some(token) = &self.token {
            if token.is(TokenType::EOF) {
                eprintln!("{} at end {}", token.clone().line, self.message)
            } else {
                eprintln!("{} at '{}' {}", token.clone().line, token.clone().lexeme, self.message)
            }
        }
        eprintln!("[line {}] Error{}: {}", self.line, loc, self.message)
    }
}