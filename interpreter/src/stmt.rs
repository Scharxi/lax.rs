use crate::error::*;
use crate::expr::Expr;
use crate::token::*;

pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
}
impl Stmt {
    pub fn accept<T>(&self, stmt_visitor: &dyn StmtVisitor<T>) -> Result<T, LaxError> {
        match self {
            Stmt::Expression(v) => v.accept(stmt_visitor),
            Stmt::Print(v) => v.accept(stmt_visitor),
        }
    }
}

pub struct ExpressionStmt {
    pub expression: Expr,
}

pub struct PrintStmt {
    pub expression: Expr,
}
pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&self, expr: &ExpressionStmt) -> Result<T, LaxError>;
    fn visit_print_stmt(&self, expr: &PrintStmt) -> Result<T, LaxError>;
}

impl ExpressionStmt {
    fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LaxError> {
        visitor.visit_expression_stmt(self)
    }
}

impl PrintStmt {
    fn accept<T>(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, LaxError> {
        visitor.visit_print_stmt(self)
    }
}
