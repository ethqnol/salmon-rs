#![ allow(warnings)]

use ast::ast::AstPrinter;
use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
use token::TokenType;
mod scope;
mod ast;
mod interpreter;
mod error;
mod lexer;
mod function;
mod parser;
mod token;
mod object;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });
    match command.as_str() {
        "tokenize" => {
            if !file_contents.is_empty() {
                let mut lexer: Lexer = Lexer::new(file_contents.as_str());
                lexer.tokenize();
                lexer.get_tokens().into_iter().for_each(|token| {
                    if token.token_type == TokenType::STRING {
                        println!(
                            "{:?} \"{}\" {}",
                            token.token_type, token.lexeme, token.lexeme
                        );
                        return;
                    } else if token.token_type == TokenType::NUMBER {
                        if token.lexeme.ends_with(".0") {
                            println!(
                                "{:?} {} {}",
                                token.token_type,
                                token.lexeme.replace(".0", ""),
                                token.lexeme
                            );
                        } else if token.lexeme.ends_with(".00") {
                            println!(
                                "{:?} {} {}.0",
                                token.token_type,
                                token.lexeme,
                                token.lexeme.replace(".00", "")
                            );
                        } else if !token.lexeme.contains(".") {
                            println!("{:?} {} {}.0", token.token_type, token.lexeme, token.lexeme);
                        } else {
                            println!("{:?} {} {}", token.token_type, token.lexeme, token.lexeme);
                        }
                    } else {
                        println!("{:?} {} null", token.token_type, token.lexeme)
                    }
                });

                if lexer.num_errors > 0 {
                    exit(65);
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        "interp" => {
            if !file_contents.is_empty() {
                let mut lexer: Lexer = Lexer::new(file_contents.as_str());
                lexer.tokenize();

                let mut parser : Parser = Parser::new(lexer.get_tokens());
                let mut interpreter = interpreter::Interpreter::new();
                match parser.parse() {
                    Ok(stmts) => { interpreter.interpret(&stmts); }
                    Err(e) => {
                        writeln!(io::stderr(), "{}", e).unwrap();
                    }
                }
                
                if parser.error_count > 0 {
                    exit(65);
                }
            }
        }
        
        "interp-expr" => {
            if !file_contents.is_empty() {
                let mut lexer: Lexer = Lexer::new(file_contents.as_str());
                lexer.tokenize();

                let mut parser: Parser = Parser::new(lexer.get_tokens());
                let mut interpreter = interpreter::Interpreter::new();
                match parser.parse_expr() {
                    Ok(expr) => {
                        match interpreter.evaluate(&expr) {
                            Ok(obj) => {
                                println!("{}", obj);
                            }
                            Err(e) => {
                                writeln!(io::stderr(), "{}", e).unwrap();
                            }
                        }
                    }
                    Err(e) => {
                        writeln!(io::stderr(), "{}", e).unwrap();
                    }
                }
                
                if parser.error_count > 0 {
                    exit(65);
                }
            }
        }
        
        "parse" => {
            if !file_contents.is_empty() {
                let mut lexer: Lexer = Lexer::new(file_contents.as_str());
                lexer.tokenize();

                let mut parser: Parser = Parser::new(lexer.get_tokens());
                let ast = parser.parse();
                match ast {
                    Ok(ast) => {
                        println!("{}", AstPrinter::print_stmt(ast));
                    }
                    Err(e) => {
                        writeln!(io::stderr(), "{}", e).unwrap();
                    }
                }
                if parser.error_count > 0 {
                    exit(65);
                }
            }
        }
        
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();

            return;
        }
    }
}
