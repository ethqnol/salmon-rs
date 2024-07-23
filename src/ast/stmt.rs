use crate::token::Token;
use super::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block {
        stmts: Vec<Stmt>,
    },
    
    Expression {
        expr: Expr,
    },
    
    Class {
        name: Token,
        methods: Vec<Stmt>,
    },
    
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    
    Print {
        expr: Expr,
    },
    
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

pub trait Visitor<R> {
    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> R;
    fn visit_expression(&mut self, expr: &Expr) -> R;
    fn visit_class(&mut self, name: &Token, methods: &Vec<Stmt>) -> R;
    fn visit_function(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> R;
    fn visit_if(&mut self, condition: &Expr, then_branch: &Stmt, else_branch: &Option<Box<Stmt>>) -> R;
    fn visit_print(&mut self, expr: &Expr) -> R;
    fn visit_return(&mut self, keyword: &Token, value: &Option<Expr>) -> R;
    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> R;
    fn visit_while(&mut self, condition: &Expr, body: &Stmt) -> R;
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut impl Visitor<R>) -> R {
        match self {
            Stmt::Block { stmts } => visitor.visit_block(stmts),
            Stmt::Expression { expr } => visitor.visit_expression(expr),
            Stmt::Class { name, methods } => visitor.visit_class(name, methods),
            Stmt::Function { name, params, body } => visitor.visit_function(name, params, body),
            Stmt::If { condition, then_branch, else_branch } => visitor.visit_if(condition, then_branch, else_branch),
            Stmt::Print { expr } => visitor.visit_print(expr),
            Stmt::Return { keyword, value } => visitor.visit_return(keyword, value),
            Stmt::Var { name, initializer } => visitor.visit_var(name, initializer),
            Stmt::While { condition, body } => visitor.visit_while(condition, body),
        }
    }
}
