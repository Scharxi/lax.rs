use crate::{
    error::LaxError,
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    stmt::{ExpressionStmt, PrintStmt, Stmt},
    token::{Object, Token, TokenType},
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_) => None,
        }
    }

    pub fn parse_statement(&mut self) -> Result<Vec<Stmt>, LaxError> {
        let mut stmts: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            stmts.push(self.statement()?);
        }
        Ok(stmts)
    }

    fn statement(&mut self) -> Result<Stmt, LaxError> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }
        self.expr_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LaxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Print(PrintStmt { expression: expr }))
    }

    fn expr_statement(&mut self) -> Result<Stmt, LaxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(ExpressionStmt { expression: expr }))
    }

    fn expression(&mut self) -> Result<Expr, LaxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LaxError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::Equal, TokenType::BangEqual, TokenType::BangIn]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LaxError> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::EqualEqual,
            TokenType::In,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LaxError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LaxError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LaxError> {
        if self.match_token(&[
            TokenType::Bang,
            TokenType::Minus,
            TokenType::Not,
            TokenType::Plus,
        ]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LaxError> {
        if self.match_token(&[TokenType::True]) {
            Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(true)),
            }))
        } else if self.match_token(&[TokenType::False]) {
            Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Bool(false)),
            }))
        } else if self.match_token(&[TokenType::Nil]) {
            Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }))
        } else if self.match_token(&[TokenType::Number, TokenType::String]) {
            Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }))
        } else if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParent, "Expect ')' after expression.")?;
            Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }))
        } else {
            Err(LaxError::error(
                self.peek().line,
                "Expected expression".to_string(),
            ))
        }
    }

    fn consume(&mut self, t_type: TokenType, message: &str) -> Result<Token, LaxError> {
        if self.check(t_type) {
            return Ok(self.advance());
        }
        Parser::error(&self.peek(), message)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().is(TokenType::Semicolon) {
                return;
            }
            if matches!(
                self.peek().t_type,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }
        }
    }

    fn error(token: &Token, message: &str) -> Result<Token, LaxError> {
        Err(LaxError::parse_error(token.clone(), message))
    }

    fn previous(&mut self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, t_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().t_type == t_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().t_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }
}
