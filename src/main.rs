#![allow(unused)]

mod lexer_utils;
mod parser_utils;
mod repl;
mod object;

fn main() {
    repl::start();
}
