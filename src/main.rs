mod parser;
mod lexer;
mod program;
mod util;

use std::fs;
use crate::lexer::{TokenStream};
use crate::parser::Parser;

fn main() {
    let input = fs::read_to_string("./.example/index.mp")
        .expect("Should have been able to read the file");

    let stream = TokenStream::new(input).unwrap();

    let mut parser = Parser::new_from_stream(stream);

    let tree = parser.parse_program().unwrap();

    println!("{}", tree.format(0))
}