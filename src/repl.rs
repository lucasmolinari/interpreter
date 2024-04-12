use crate::evaluator_utils::evaluator::eval;
use crate::parser_utils::parser::Parser;
use crate::{evaluator_utils::environment::Environment, lexer_utils::lexer::Lexer};
use std::io::{self, Write};

pub fn start() {
    println!("q! for exit.");
    let mut env = Environment::new();
    loop {
        let mut input = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read input");

        if input.trim() == "q!".to_string() {
            std::process::exit(0);
        }

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();

        if p.errors().len() != 0 {
            print_parse_errors(p.errors());
            continue;
        }
        let evaluated = eval(&program.statements, &mut env);
        println!("{}", evaluated.inspect());
    }
}

fn print_parse_errors(errors: &Vec<String>) {
    println!("Parser Errors:");
    for e in errors {
        println!("\t{}", e);
    }
}
