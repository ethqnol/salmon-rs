use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Unary {
        op: Token,
        value: Box<Expr>,
    },

    Binary {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    Grouping {
        expr: Box<Option<Expr>>,
    },

    Literal {
        value: Token,
    },
}

pub trait Visitor<R> {
    fn visit_unary(&self, op: &Token, value: &Box<Expr>) -> R;
    fn visit_binary(&self, op: &Token, left: &Box<Expr>, right: &Box<Expr>) -> R;
    fn visit_grouping(&self, expr: &Box<Option<Expr>>) -> R;
    fn visit_literal(&self, value: &Token) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &impl Visitor<R>) -> R {
        match self {
            Expr::Unary { op, value } => visitor.visit_unary(op, value),
            Expr::Binary { op, left, right } => visitor.visit_binary(op, left, right),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Grouping { expr } => visitor.visit_grouping(expr),
        }
    }
}
