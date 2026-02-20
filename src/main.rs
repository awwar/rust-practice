mod compiler;
mod parser;
mod lexer;
mod program;
mod util;
mod procedure;
mod vm;

use crate::compiler::Compiler;
use crate::lexer::TokenStream;
use crate::parser::Parser;
use crate::vm::execute;
use std::fs;

fn main() {
    let input = fs::read_to_string("./.example/index.mp")
        .expect("Should have been able to read the file");

    let stream = TokenStream::new(input).unwrap();

    let mut parser = Parser::new_from_stream(stream);

    let tree = parser.parse_program().unwrap();

    println!("{}", tree.format(0));

    let mut compiler = Compiler::new();

    compiler.compile(tree).unwrap();

    let prog = &mut compiler.program;

    println!("{}", prog.to_string());

    execute(prog)
}