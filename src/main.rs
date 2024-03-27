#![allow(unused)]

mod lexer_utils;
mod parser_utils;

use lexer_utils::repl;
use lexer_utils::lexer::Lexer;

use parser_utils::parser::Parser;

fn main() {
    // repl::start();
    let l = Lexer::new("let x = 5;
    let y = 10;
    let foobar = 838383;".to_string());
    let mut p = Parser::new(l);
    
    let r = p.parse_program();
    for stmt in r.statements { 
        println!("{:?}", stmt)
    }
}
