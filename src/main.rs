// #![allow(unused)]

mod lexer_utils;
mod parser_utils;

// use lexer_utils::repl;
use lexer_utils::lexer::Lexer;

use parser_utils::parser::Parser;

fn main() {
    // repl::start();
    let l = Lexer::new("foobar; barfoo".to_string());
    let mut p = Parser::new(l);
    
    let prg = p.parse_program();
    
    for stmt in prg.statements {
        println!("{:?}", stmt)
    }

    let e = p.errors();
    if e.len() > 0 {
        println!("Parser has {} errors:\n{:?}", e.len(), e);
    } else {
        println!("Parser has no errors!")
    }
}
