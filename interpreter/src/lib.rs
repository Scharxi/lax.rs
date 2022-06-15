use error::*;
use token::*;

pub mod error;
pub mod expr;
pub mod interpreter;
pub mod lox;
pub mod parser;
pub mod printer;
pub mod scanner;
pub mod token;

#[cfg(test)]
pub mod tests {
    use crate::{
        expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr},
        token::{Object, Token, TokenType},
    };

    #[test]
    fn test_ast_printer() {
        use crate::expr::UnaryExpr;
        use crate::printer::AstPrinter;

        let expr = Expr::Binary(BinaryExpr {
            left: Box::new(Expr::Unary(UnaryExpr {
                operator: Token {
                    t_type: TokenType::Minus,
                    lexeme: "-".to_string(),
                    literal: None,
                    line: 1,
                },
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(Object::Num(123.0)),
                })),
            })),
            operator: Token {
                t_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Grouping(GroupingExpr {
                expression: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(Object::Num(45.67)),
                })),
            })),
        });

        let printer = AstPrinter {};
        assert_eq!(
            printer.print(&expr).unwrap(),
            "(* (- 123) (group 45.67))".to_string()
        );
        println!("{}", printer.print(&expr).unwrap());
    }
}
