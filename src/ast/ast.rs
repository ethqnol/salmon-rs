use super::expr::*;
use crate::token::{Token, TokenType};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(expr: Expr) -> String {
        expr.accept(&Self)
    }
}

impl AstPrinter {
    pub fn parenthesize(&self, name: Token, exprs: &[Expr]) -> String {
        let mut parenthesized_expr: String = String::from(format!("({}", name.lexeme));

        for expr in exprs {
            parenthesized_expr.push(' ');
            parenthesized_expr.push_str(&format!("{}", expr.accept(self)));
        }
        parenthesized_expr.push_str(")");
        parenthesized_expr
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_unary(&self, op: &Token, value: &Box<Expr>) -> String {
        self.parenthesize(op.clone(), &[*value.clone()])
    }

    fn visit_binary(&self, op: &Token, left: &Box<Expr>, right: &Box<Expr>) -> String {
        self.parenthesize(op.clone(), &[*left.clone(), *right.clone()])
    }

    fn visit_literal(&self, value: &Token) -> String {
        match value.token_type {
            TokenType::NULL => format!("{}", "null"),
            TokenType::NUMBER => {
                if !value.lexeme.contains('.') {
                    format!("{}.0", value.lexeme)
                } else {
                    value.lexeme.to_string()
                }
            }  
            TokenType::TRUE => format!("{}", "true"),
            TokenType::FALSE => format!("{}", "false"),
            TokenType::STRING => value.lexeme.to_string(),
            _ => format!("no such literal"),
        }
    }

    fn visit_grouping(&self, expr: &Box<Option<Expr>>) -> String {
        if expr.is_none() {
            return String::from("");
        }
        let new_expr = &expr.clone().unwrap();
        format!("(group {})", new_expr.accept(self))
    }
}
