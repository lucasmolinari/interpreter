// #![allow(unused)]

mod lexer_utils;
mod parser_utils;

// use lexer_utils::repl;
use lexer_utils::lexer::Lexer;

use parser_utils::parser::Parser;

fn main() {
    // repl::start();
    let l = Lexer::new("let x = 5;
    let {} = 10;
    let foobar = 838383;".to_string());
    let mut p = Parser::new(l);
    
    p.parse_program();
    let e = p.errors();
    println!("Parser has {} errors:\n{:?}", e.len(), e);
}
