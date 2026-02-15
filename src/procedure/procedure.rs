use crate::compiler::SubCompiler;
use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::program::{Program, ValueConverter};
use std::collections::LinkedList;

pub trait Procedure {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        Ok(Node::new_operation(token.value, vec![], token.at))
    }
    fn compile(&self, prog: Program, node: Node, sub_compiler: impl SubCompiler) -> Option<String>
    where
        Self: Sized,
    {
        sub_compiler.compile(node)
    }
    fn execute(&self, argc: usize, stack: LinkedList<Box<dyn ValueConverter>>) -> Option<String> {
        panic!("procedure not implemented yet");
    }
}