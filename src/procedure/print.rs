use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::Procedure;
use crate::vm::Stack;

pub struct Print {}

impl Procedure for Print {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // PRINT (expression)
        let expr = parser.subparse_one_in_bracers()?;

        Ok(Node::new_operation(token.value, vec![expr], token.at))
    }
    fn execute(&self, argc: usize, _stack: &mut Stack) -> Result<(), String> {
        if argc != 1 {
            return Err(String::from("argument count must be 1"));
        }

        // let first_operand = stack.pop();
        // //
        // println!("{}", first_operand.raw());

        Ok(())
    }
}