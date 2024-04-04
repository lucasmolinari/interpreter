#![allow(unused)]

mod tests;
mod lexer_utils;
mod parser_utils;


use lexer_utils::lexer::Lexer;
use parser_utils::parser::Parser;
use lexer_utils::repl;
    
fn main() {
    // repl::start();
    let input = String::from("12 * 43;");
    let mut l = Lexer::new(input);
    let mut p = Parser::new(l);
    let program = p.parse_program();
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("{} STATEMENT: {:?}", i, stmt); 
    }
}