use super::{expr, stmt};
use super::expr::*;
use super::stmt::*;
use crate::token::{Token, TokenType};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print_expr(expr: Expr) -> String {
        expr.accept(&mut Self)
    }
    
    pub fn print_stmt(stmts: Vec<Stmt>) -> String {
        let mut final_str = String::new();
        for mut stmt in stmts {
            final_str.push_str(&stmt.accept(&mut Self));
                final_str.push('\n');
        }
        final_str
    }
}

impl AstPrinter {
    pub fn parenthesize(&mut self, name: Token, exprs: &[Expr]) -> String {
        let mut parenthesized_expr: String = String::from(format!("({}", name.lexeme));

        for expr in exprs {
            parenthesized_expr.push(' ');
            parenthesized_expr.push_str(&format!("{}", expr.accept(self)));
        }
        parenthesized_expr.push_str(")");
        parenthesized_expr
    }
    
}

impl expr::Visitor<String> for AstPrinter {
    fn visit_unary(&mut self, op: &Token, value: &Box<Expr>) -> String {
        self.parenthesize(op.clone(), &[*value.clone()])
    }

    fn visit_binary(&mut self, op: &Token, left: &Box<Expr>, right: &Box<Expr>) -> String {
        self.parenthesize(op.clone(), &[*left.clone(), *right.clone()])
    }

    fn visit_literal(&mut self, value: &Token) -> String {
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
            TokenType::STRING => format!("\"{}\"", value.lexeme),
            _ => format!("no such literal"),
        }
    }

    fn visit_grouping(&mut self, expr: &Box<Expr>) -> String {
        let new_expr = (*expr).clone();
        format!("(group {})", new_expr.accept(self))
    }
    
    fn visit_variable(&mut self, name: &Token) -> String {
        name.lexeme.to_string()
    }
    
    fn visit_assign(&mut self, name: &Token, value: &Box<Expr>) -> String {
        format!("(= {} {})", name.lexeme, value.accept(self))
    }
    
    fn visit_logical(&mut self, op: &Token, left: &Box<Expr>, right: &Box<Expr>) -> String {
        self.parenthesize(op.clone(), &[*left.clone(), *right.clone()])
    }
    
    fn visit_call(&mut self, callee: &Box<Expr>, paren: &Token, arguments: &Vec<Expr>) -> String {
        let mut call_expr = String::from(format!("({}", callee.accept(self)));
        call_expr.push_str(" (");
        call_expr.push_str(&paren.lexeme);
        for arg in arguments {
            call_expr.push_str(&format!(" {}", arg.accept(self)));
        }
        call_expr.push_str(")");
        call_expr
    }
    
    // fn visit_this(&mut self, keyword: &Token) -> String {
    //     "this".to_string()
    // }
    
    // fn visit_super(&mut self, keyword: &Token, method: &Token) -> String {
    //     format!("(super {})", method.lexeme)
    // }
    
    // fn visit_set(&mut self, object: &Box<Expr>, name: &Token, value: &Box<Expr>) -> String {
    //     format!("(set {} {} {})", object.accept(self), name.lexeme, value.accept(self))
    // }
    
    // fn visit_get(&mut self, object: &Box<Expr>, name: &Token) -> String {
    //     format!("(get {} {})", object.accept(self), name.lexeme)
    // }
}

impl stmt::Visitor<String> for AstPrinter {
    fn visit_block(&mut self, stmts: &Vec<Stmt>) -> String {
        let mut block_stmt = String::from("(block ");
        for mut stmt in stmts.clone() {
            block_stmt.push_str(&format!("{}", stmt.accept(self)));
        }
        block_stmt.push_str(")");
        block_stmt
    }
    
    fn visit_class(&mut self, name: &Token, methods: &Vec<Stmt>) -> String {
        let mut class_stmt = String::from(format!("(class {} ", name.lexeme));
        for mut method in methods.clone() {
            class_stmt.push_str(&format!("{}", method.accept(self)));
        }
        class_stmt.push_str(")");
        class_stmt
    }
    
    fn visit_expression(&mut self, expr: &Box<Expr>) -> String {
        format!("(; {})", expr.accept(self))
    }
    
    fn visit_function(&mut self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> String {
        let mut function_stmt = String::from(format!("(fun {} (", name.lexeme));
        for param in params {
            function_stmt.push_str(&format!(" {}", param.lexeme));
        }
        function_stmt.push_str(") ");
        for mut stmt in body.clone() {
            function_stmt.push_str(&format!("{}", stmt.accept(self)));
        }
        function_stmt.push_str(")");
        function_stmt
    }
    
    fn visit_if(&mut self, condition: &Box<Expr>, then_branch: &Box<Stmt>, else_branch: &Option<Box<Stmt>>) -> String {
        let mut if_stmt = String::from("(if ");
        if_stmt.push_str(&format!("{}", condition.accept(self)));
        if_stmt.push_str(&format!("{}", then_branch.clone().accept(self)));
        if else_branch.is_some() {
            if_stmt.push_str(&format!("{}", else_branch.clone().unwrap().accept(self)));
        }
        if_stmt.push_str(")");
        if_stmt
    }
    
    fn visit_print(&mut self, expr: &Box<Expr>) -> String {
        format!("(print {})", expr.accept(self))
    }
    
    fn visit_return(&mut self, keyword: &Token, value: &Option<Expr>) -> String {
        if value.is_none() {
            return String::from("(return)");
        }
        
        format!("(return {})", value.clone().unwrap().accept(self))
    }
    
    fn visit_var(&mut self, name: &Token, initializer: &Option<Expr>) -> String {
        if initializer.is_none() {
            return format!("(var {})", name.lexeme);
        }
        
        format!("(var {} {})", name.lexeme, initializer.clone().unwrap().accept(self))
    }
    
    fn visit_while(&mut self, condition: &Box<Expr>, body: &Box<Stmt>) -> String {
        let mut while_stmt = String::from("(while ");
        while_stmt.push_str(&format!("{}", condition.accept(self)));
        while_stmt.push_str(&format!("{}", body.clone().accept(self)));
        while_stmt.push_str(")");
        while_stmt
    }    
} 