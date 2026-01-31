mod lex;
mod parser;
mod program;

use std::fs;
use crate::lex::{Lexer, Token};

fn main() {
    let input = fs::read_to_string("./.example/index.mp")
        .expect("Should have been able to read the file");

    let lexer = Lexer::new(input);

    lexer.for_each(|node: Token| println!("{}", node.to_string()));
}