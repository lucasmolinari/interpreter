// #![allow(unused)]

mod lexer_utils;
mod parser_utils;
mod tests;

// use lexer_utils::repl;
use tests::TestError;
    
fn main() {
    // repl::start();
    let mut t = TestError::new();
    let results = t.test_all();
    for r in results {
        match r {
            Ok(s) => println!("{}", s),
            Err(e) => println!("{}", e),
        }
    }
}