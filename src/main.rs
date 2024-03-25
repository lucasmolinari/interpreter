// #![allow(unused)]

mod utils;
use utils::lexer::Lexer;

use crate::utils::token::TokenType;
fn main() {
    let test_string = "let five = 5;
    let ten = 10;
    let add = fn(x, y) {
    x + y;
    };
    let result = add(five, ten);";
    let mut l = Lexer::new(test_string.to_string());
    loop {
        let tok = l.next_token();
        println!("{:?}", tok);
        if tok.token_type == TokenType::EOF {
            break;
        };
    }
}
