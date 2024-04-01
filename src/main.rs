#![allow(unused)]

mod tests;
mod lexer_utils;
mod parser_utils;


use lexer_utils::lexer::Lexer;
use parser_utils::parser::Parser;
use lexer_utils::repl;
    
fn main() {
    repl::start();
}