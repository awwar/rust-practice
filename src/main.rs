mod parser;
mod lexer;
mod program;

use std::fs;
use crate::lexer::{TokenStream};

fn main() {
    let input = fs::read_to_string("./.example/index.mp")
        .expect("Should have been able to read the file");

    let mut stream = TokenStream::new(input)
        .unwrap();

    let mut i: usize = 0;

    loop {
        let candidate = stream.get(i);

        if candidate.is_none() {
            break;
        }
        i += 1;

        println!("{}", candidate.unwrap().to_string())
    }
}