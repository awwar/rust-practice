use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::Procedure;
use crate::vm::Stack;

pub struct Print {}

impl Procedure for Print {
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        if argc != 1 {
            return Err(String::from("argument count must be 1"));
        }

        let _ = stack.pop();
        // println!("{}", first_operand.repr());

        Ok(())
    }
}