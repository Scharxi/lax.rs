use crate::{LaxError, Object, Token, TokenType};

pub struct Scanner {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, LaxError> {
        let mut had_error: Option<LaxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(err) => {
                    err.report("");
                    had_error = Some(err)
                }
            }
        }

        self.tokens.push(Token::eof(self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(self.tokens)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LaxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParent, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let tok = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tok, None);
            }
            '=' => {
                let tok = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(tok, None);
            }
            '>' => {
                let tok = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tok, None);
            }
            '<' => {
                let tok = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tok, None);
            }
            '/' => {
                if self.matches('/') {
                    // Comment goes until the end of the line
                    while let Some(ch) = self.peek() {
                        if ch != '\n' {
                            self.advance();
                        } else {
                            break;
                        };
                    }
                } else if self.matches('*') {
                    // block comment start
                    self.scan_comment()?;
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => {
                self.string()?;
            }
            '0'..='9' => {
                self.number()?;
            }
            _ if c.is_ascii_alphabetic() || c == '_' => {
                self.identifier();
            },
            _ => {
                return Err(LaxError::error(self.line, "Unexpected character.".to_string()));
            }
        }
        Ok(())
    }

    fn matches(&mut self, expected: char) -> bool {
        match self.source.get(self.current).copied() {
            Some(ch) if ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn advance(&mut self) -> char {
        let res = self.source.get(self.current).copied().unwrap();
        self.current += 1;
        res
    }

    fn add_token(&mut self, t_type: TokenType, literal: Option<Object>) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(t_type, text, literal, self.line));
    }

    fn string(&mut self) -> Result<(), LaxError> {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    break;
                }
                '\n' => {
                    self.line += 1;
                }
                _ => {}
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(LaxError::error(self.line, "Unterminated string".to_string()));
        }

        // The closing ".
        self.advance();

        // TODO: Handle Escape Sequences
        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token(TokenType::String, Some(Object::Str(value)));
        Ok(())
    }

    fn scan_comment(&mut self) -> Result<(), LaxError> {
        loop {
            match self.peek() {
                Some('*') => {
                    self.advance();
                    if self.matches('/') {
                        return Ok(());
                    }
                },
                Some('/') => {
                    self.advance();
                    if self.matches('*') {
                        self.scan_comment();
                    }
                }
                Some('\n') => {
                    self.advance();
                    self.line += 1;
                }
                None => {
                    return Err(LaxError::error(self.line, "Unterminated comment".to_string()))
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn number(&mut self) -> Result<(), LaxError> {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        self.add_token(TokenType::Number, Some(Object::Num(value.parse::<f64>().unwrap())));
        Ok(())
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        if let Some(t_type) = Scanner::keyword(value.as_str()) {
            self.add_token(t_type, None);
        } else {
            self.add_token(TokenType::Identifier, None)
        }
    }

    fn keyword(check: &str) -> Option<TokenType> {
        match check {
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "if" => Some(TokenType::If),
            "false" => Some(TokenType::False),
            "true" => Some(TokenType::True),
            "fun" => Some(TokenType::Fun),
            "for" => Some(TokenType::For),
            "loop" => Some(TokenType::Loop),
            "while" => Some(TokenType::While),
            "break" => Some(TokenType::Break),
            "continue" => Some(TokenType::Continue),
            "this" => Some(TokenType::This),
            "super" => Some(TokenType::Super),
            "nil" => Some(TokenType::Nil),
            "var" => Some(TokenType::Var),
            "val" => Some(TokenType::Val),
            "not" => Some(TokenType::Not),
            "or" => Some(TokenType::Or),
            "and" => Some(TokenType::And),
            "print" => Some(TokenType::Print),
            "is" => Some(TokenType::Is),
            "in" => Some(TokenType::In),
            "return" => Some(TokenType::Return),
            _ => None
        }
    }

    fn is_alpha_numeric(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_alphanumeric()
        } else {
            false
        }
    }

    fn is_digit(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_digit()
        } else {
            false
        }
    }

    fn peek_next(&mut self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }
}