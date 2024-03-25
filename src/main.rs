// #![allow(unused)]

mod utils;
use utils::{lexer::Lexer, token::Keywords, token::TokenType};

fn main() {
    let test_string = "let five = 5;
    let ten = 10;
    let add = fn(x, y) {
    x + y;
    };
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    if (5 < 10){
        return true;
    } else {
        return false;
    }
    10 == 10;
    10 != 9;
    ";
    let mut l = Lexer::new(test_string.to_string());
    let keywords = Keywords::default();
    loop {
        let tok = l.next_token(&keywords);
        println!("{:?}", tok);
        if tok.token_type == TokenType::EOF {
            break;
        };
    }
}
