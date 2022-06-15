use crate::error::*;
use crate::token::*;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}
impl Expr {
    pub fn accept<T>(&self, expr_visitor: &dyn ExprVisitor<T>) -> Result<T, LaxError> {
        match self {
            Expr::Binary(v) => v.accept(expr_visitor),
            Expr::Grouping(v) => v.accept(expr_visitor),
            Expr::Literal(v) => v.accept(expr_visitor),
            Expr::Unary(v) => v.accept(expr_visitor),
        }
    }
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: Option<Object>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct DoubleExpr {
    pub left: Box<Expr>,
    pub operator_one: Token,
    pub operator_2: Token,
    pub right: Box<Expr>,
}
pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LaxError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LaxError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LaxError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LaxError>;
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LaxError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LaxError> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LaxError> {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LaxError> {
        visitor.visit_unary_expr(self)
    }
}
