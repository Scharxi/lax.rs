//! This module contains the logic to evaluate expressions

use crate::{
    error::LaxError,
    expr::{Expr, ExprVisitor},
    stmt::{Stmt, StmtVisitor},
    token::{Object, TokenType},
};

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, expr: &Expr) -> Result<Object, LaxError> {
        let value = self.evaluate(&expr)?;
        println!("{}", value);
        Ok(value)
    }

    pub fn interpret_statement(&self, stmts: &Vec<Stmt>) -> Result<(), LaxError> {
        for stmt in stmts {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), LaxError> {
        stmt.accept(self)
    }

    /// Evaluates the given expression by calling the appropriate visitor method.
    ///
    /// Returns the result of the evaluation.
    fn evaluate(&self, expr: &Expr) -> Result<Object, LaxError> {
        expr.accept(self)
    }

    /// Checks if the given object is truthy or falsey.
    /// # Returns
    /// `false` if the object is nil
    ///
    /// `true` or `false` depending on the [`Object::Bool`] value
    ///
    /// `true` or `false` for [`Object::Str`] if the string is not empty
    ///
    /// `true` for everything else
    fn is_truthy(&self, object: &Object) -> bool {
        match object {
            Object::Nil => false,
            Object::Bool(bool) => *bool,
            Object::Str(str) => str.is_empty(),
            _ => true,
        }
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_binary_expr(
        &self,
        expr: &crate::expr::BinaryExpr,
    ) -> Result<Object, crate::error::LaxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match &expr.operator.t_type {
            TokenType::Minus => match left {
                Object::Num(left) => match right {
                    Object::Num(right) => Ok(Object::Num(left - right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for -: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for -: {:?} and {:?}", left, right),
                )),
            },
            TokenType::Slash => match left {
                Object::Num(left) => match right {
                    Object::Num(right) => Ok(Object::Num(left / right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for /: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for /: {:?} and {:?}", left, right),
                )),
            },
            TokenType::Star => match left {
                Object::Num(left) => match right {
                    Object::Num(right) => Ok(Object::Num(left * right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for *: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for *: {:?} and {:?}", left, right),
                )),
            },
            TokenType::Plus => match left {
                Object::Num(left) => match right {
                    Object::Num(right) => Ok(Object::Num(left + right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for +: {:?} and {:?}", left, right),
                    )),
                },
                Object::Str(left) => match right {
                    Object::Str(right) => Ok(Object::Str(format!("{}{}", left, right))),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for +: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for +: {:?} and {:?}", left, right),
                )),
            },
            TokenType::Greater => match left {
                Object::Num(left) => match right {
                    Object::Num(right) => Ok(Object::from(left > right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for >: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for >: {:?} and {:?}", left, right),
                )),
            },
            TokenType::GreaterEqual => match left {
                Object::Num(left) => match right {
                    Object::Num(right) => Ok(Object::from(left >= right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for >=: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for >=: {:?} and {:?}", left, right),
                )),
            },
            TokenType::Less => match left {
                Object::Num(left) => match right {
                    Object::Num(right) => Ok(Object::from(left < right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for <: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for <: {:?} and {:?}", left, right),
                )),
            },
            TokenType::LessEqual => match left {
                Object::Num(left) => match right {
                    Object::Num(right) => Ok(Object::from(left <= right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for <=: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for <=: {:?} and {:?}", left, right),
                )),
            },
            TokenType::BangEqual => match left {
                Object::Num(left) => match right {
                    Object::Num(right) => Ok(Object::from(left != right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for !=: {:?} and {:?}", left, right),
                    )),
                },
                Object::Str(left) => match right {
                    Object::Str(right) => Ok(Object::from(left != right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for !=: {:?} and {:?}", left, right),
                    )),
                },
                Object::Bool(left) => match right {
                    Object::Bool(right) => Ok(Object::from(left != right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for !=: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for !=: {:?} and {:?}", left, right),
                )),
            },
            TokenType::EqualEqual => match left {
                Object::Num(left) => match right {
                    Object::Num(right) => Ok(Object::from(left == right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for ==: {} and {}", left, right),
                    )),
                },
                Object::Str(left) => match right {
                    Object::Str(right) => Ok(Object::from(left == right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for ==: {} and {}", left, right),
                    )),
                },
                Object::Bool(left) => match right {
                    Object::Bool(right) => Ok(Object::from(left == right)),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for ==: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for ==: {} and {}", left, right),
                )),
            },
            TokenType::In => match left {
                Object::Str(left) => match right {
                    Object::Str(right) => Ok(Object::from(right.contains(&left))),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for in: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for in: {:?} and {:?}", left, right),
                )),
            },
            TokenType::BangIn => match left {
                Object::Str(left) => match right {
                    Object::Str(right) => Ok(Object::from(!right.contains(&left))),
                    _ => Err(LaxError::error(
                        expr.operator.line,
                        format!("Invalid operands for !in: {:?} and {:?}", left, right),
                    )),
                },
                _ => Err(LaxError::error(
                    expr.operator.line,
                    format!("Invalid operands for !in: {:?} and {:?}", left, right),
                )),
            },
            _ => Err(LaxError::error(
                expr.operator.line,
                format!("Invalid operator: {:?}", expr.operator.t_type),
            )),
        }
    }

    fn visit_grouping_expr(
        &self,
        expr: &crate::expr::GroupingExpr,
    ) -> Result<Object, crate::error::LaxError> {
        Ok(self.evaluate(&expr.expression)?)
    }

    fn visit_literal_expr(
        &self,
        expr: &crate::expr::LiteralExpr,
    ) -> Result<Object, crate::error::LaxError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_unary_expr(
        &self,
        expr: &crate::expr::UnaryExpr,
    ) -> Result<Object, crate::error::LaxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.t_type {
            TokenType::Minus => match right {
                Object::Num(num) => return Ok(Object::Num(-num)),
                _ => return Ok(Object::Nil),
            },
            TokenType::Bang | TokenType::Not => return Ok(Object::from(!self.is_truthy(&right))),
            TokenType::Plus => {
                match right {
                    Object::Str(value) => {
                        // return Ok(Object::Num(value.parse().unwrap()))
                        if let Ok(num) = value.parse::<f64>() {
                            return Ok(Object::Num(num));
                        } else {
                            return Err(LaxError::error(
                                expr.operator.line,
                                format!("Could not parse {} to a number", value),
                            ));
                        }
                    }
                    _ => {
                        return Err(LaxError::error(
                            expr.operator.line,
                            format!(
                                "Invalid operands for +: {:?} and {:?}",
                                right, expr.operator.t_type
                            ),
                        ))
                    }
                }
            }
            _ => return Err(LaxError::error(0, "Unreachable".to_owned())),
        };
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, expr: &crate::stmt::ExpressionStmt) -> Result<(), LaxError> {
        self.evaluate(&expr.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, expr: &crate::stmt::PrintStmt) -> Result<(), LaxError> {
        let value = self.evaluate(&expr.expression)?;
        println!("{}", value);
        Ok(())
    }
}
