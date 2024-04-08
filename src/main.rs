#![allow(unused)]

mod lexer_utils;
mod parser_utils;
mod tests;

use lexer_utils::lexer::Lexer;
use lexer_utils::repl;
use parser_utils::parser::Parser;

use std::fs;

fn main() {
    // repl::start();
    let input = fs::read_to_string("./ex.crb");
    let mut l = Lexer::new(input.unwrap());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    let stmts = program.statements;
    let body = stmts[0].get_statement_expr().expression.get_function_expr().body.clone();
    for (i ,stmt) in body.statements.iter().enumerate() {
        println!("{} {:?}", i, stmt);
    }
}
