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
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("{} {:?}", i, stmt.get_statement_expr().expression.string());
    }
}
