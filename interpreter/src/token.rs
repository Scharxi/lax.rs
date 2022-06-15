use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParent,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    String,
    Identifier,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Not,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    Val,
    While,
    Loop,
    Continue,
    Break,
    Is,
    In,

    EOF,
}

#[derive(Debug, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{}", x),
            Object::Str(str) => write!(f, "\"{}\"", str),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Self { t_type, lexeme, literal, line }
    }

    pub fn is(&self, ttype: TokenType) -> bool {
        self.t_type == ttype
    }

    pub fn eof(current_line: usize) -> Token {
        Token::new(
            TokenType::EOF,
            "".to_string(),
            None,
            current_line
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.t_type, self.lexeme, if let Some(literal) = &self.literal {
            literal.to_string();
        } else {
            "None".to_string();
        })
    }
}