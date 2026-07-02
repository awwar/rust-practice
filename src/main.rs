mod compiler;
mod control_structures;
mod lexer;
mod parser;
mod procedure;
mod vm;

use crate::compiler::Compiler;
use crate::lexer::TokenStream;
use crate::parser::Parser;
use crate::vm::VM;
use std::fs;
use std::time::Instant;

fn main() {
    let input =
        fs::read_to_string("./.example/index.mp").expect("Should have been able to read the file");

    let stream = TokenStream::new(input);

    let mut parser = Parser::new_from_stream(stream);

    let tree = parser.parse_program().unwrap();

    println!("{}", tree.format(0));

    let mut compiler = Compiler::new();

    compiler.compile(tree).unwrap();

    let prog = &mut compiler.program;

    println!("{}", prog.to_string());

    let vm = VM::new();
    let now = Instant::now();

    let c = 1;

    for _ in 0..c {
        vm.execute(prog);
    }

    let duration = now.elapsed();

    println!("{}ms {}ns", duration.as_millis(), duration.as_nanos());
    println!(
        "{}ms/op {}ns/op",
        duration.as_millis() / c,
        duration.as_nanos() / c
    );
}
