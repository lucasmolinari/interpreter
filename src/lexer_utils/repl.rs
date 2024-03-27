use std::io::{self, Write};
use crate::lexer_utils::lexer::Lexer;
use crate::lexer_utils::token::TokenType;

pub fn start(){
    println!("q! for exit.");
    loop {
        let mut input = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read input");
        
        if input.trim() == "q!".to_string(){
            std::process::exit(0);
        }

        let mut l = Lexer::new(input);
        
        loop {
            let tok = l.next_token();
            println!("{:?}", tok);
            if tok.token_type == TokenType::EOF {
                break;
            };
        }
    }
}