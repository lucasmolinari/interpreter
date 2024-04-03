#![allow(unused)]

mod tests;
mod lexer_utils;
mod parser_utils;


use lexer_utils::lexer::Lexer;
use parser_utils::parser::Parser;
use lexer_utils::repl;
    
fn main() {
    // repl::start();
    let input = String::from("!5;");
    let mut l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    let stmt = program.statements.get(0).unwrap();
    let expr = &stmt.get_statement_expr().expression;
    println!("{:?}", expr.get_prefix_expr());
}