use error::*;
use token::*;

pub mod scanner;
pub mod token;
pub mod error;
pub mod lox;
pub mod expr;
pub mod printer;
pub mod parser;

#[cfg(test)]
pub mod tests {
    use crate::{expr::{Expr, BinaryExpr, LiteralExpr, GroupingExpr}, token::{Token, TokenType, Object}};

    #[test]
    fn test_ast_printer() {
       use crate::printer::AstPrinter;
       use crate::expr::{UnaryExpr};
       
       let expr = Expr::Binary( 
           BinaryExpr {
               left: Box::new(
                   Expr::Unary(
                       UnaryExpr {
                            operator: Token {
                                    t_type: TokenType::Minus,
                                    lexeme: "-".to_string(),
                                    literal: None,
                                    line: 1, 
                            },
                            right: Box::new(Expr::Literal(LiteralExpr {
                                value: Some(Object::Num(123.0))
                            })),
                       }
                   )
               ),
            operator: Token {
                t_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Grouping(GroupingExpr {
                expression: Box::new(Expr::Literal(LiteralExpr {
                    value: Some(Object::Num(45.67))
                }))
            })),
           }
       );

       let printer = AstPrinter {};
       assert_eq!(printer.print(&expr).unwrap(), "(* (- 123) (group 45.67))".to_string());
       println!("{}", printer.print(&expr).unwrap());
    }
}