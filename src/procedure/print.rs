use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::Procedure;
use std::collections::LinkedList;

pub struct Print {}

impl Procedure for Print {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // PRINT (expression)
        let expr = match parser.subparse_one_in_bracers() {
            Ok(l) => l,
            Err(e) => return Err(e)
        };

        Ok(Node::new_operation(token.value, vec![expr], token.at))
    }
    fn execute(&self, argc: usize, stack: &mut LinkedList<String>) -> Result<(), String> {
        if argc != 1 {
            return Err(String::from("argument count must be 1"));
        }

        let first_operand = stack.pop_back().unwrap();

        println!("{}", first_operand);

        Ok(())
    }
}