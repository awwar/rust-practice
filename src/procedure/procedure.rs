use crate::compiler::SubCompiler;
use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::program::Program;
use std::collections::LinkedList;

pub trait Procedure {
    fn parse(&self, token: Token, _parser: &mut Parser) -> Result<Node, String> {
        Ok(Node::new_operation(token.value, vec![], token.at))
    }
    fn compile(&self, _prog: Program, node: Node, sub_compiler: impl SubCompiler) -> Option<String>
    where
        Self: Sized,
    {
        sub_compiler.compile(node)
    }
    fn execute(&self, _argc: usize, _stack: LinkedList<String>) -> Option<String> {
        panic!("procedure not implemented yet");
    }
}