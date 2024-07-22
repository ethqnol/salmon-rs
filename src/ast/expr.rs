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
        expr: Box<Expr>,
    },

    Literal {
        value: Token,
    },
    
    Variable {
        name: Token,
    },
    
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    
    // This {
    //     keyword: Token,
    // },
    
    // Super {
    //     keyword: Token,
    //     method: Token,
    // },
    
    Logical {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    
    // Set {
    //     object: Box<Expr>,
    //     name: Token,
    //     value: Box<Expr>,
    // },
    
    // Get {
    //     object: Box<Expr>,
    //     name: Token,
    // },
    
}

pub trait Visitor<R> {
    fn visit_unary(&mut self, op: &Token, value: &Box<Expr>) -> R;
    fn visit_binary(&mut self, op: &Token, left: &Box<Expr>, right: &Box<Expr>) -> R;
    fn visit_grouping(&mut self, expr: &Box<Expr>) -> R;
    fn visit_literal(&mut self, value: &Token) -> R;
    fn visit_variable(&mut self, name: &Token) -> R;
    fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> R;
    fn visit_call(&mut self, callee: &Box<Expr>, paren: &Token, arguments: &Vec<Expr>) -> R;
    // fn visit_this(&self, keyword: &Token) -> R;
    // fn visit_super(&self, keyword: &Token, method: &Token) -> R;
    fn visit_logical(&mut self, op: &Token, left: &Box<Expr>, right: &Box<Expr>) -> R;
    // fn visit_set(&self, object: &Box<Expr>, name: &Token, value: &Box<Expr>) -> R;
    // fn visit_get(&self, object: &Box<Expr>, name: &Token) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut impl Visitor<R>) -> R {
        match self {
            Expr::Unary { op, value } => visitor.visit_unary(op, value),
            Expr::Binary { op, left, right } => visitor.visit_binary(op, left, right),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Grouping { expr } => visitor.visit_grouping(expr),
            Expr::Variable { name } => visitor.visit_variable(name),
            Expr::Assign { name, value } => visitor.visit_assign(name, value),
            Expr::Call { callee, paren, arguments } => visitor.visit_call(callee, paren, arguments),
            // Expr::This { keyword } => visitor.visit_this(keyword),
            // Expr::Super { keyword, method } => visitor.visit_super(keyword, method),
            Expr::Logical { op, left, right } => visitor.visit_logical(op, left, right),
            // Expr::Set { object, name, value } => visitor.visit_set(object, name, value),
            // Expr::Get { object, name } => visitor.visit_get(object, name),
        }
    }
}
