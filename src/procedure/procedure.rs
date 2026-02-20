use crate::compiler::Compiler;
use crate::lexer::Token;
use crate::parser::{Node, Parser};
use std::collections::LinkedList;

pub trait Procedure {
    fn parse(&self, token: Token, _parser: &mut Parser) -> Result<Node, String> {
        Ok(Node::new_operation(token.value, vec![], token.at))
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        sc.sub_compile(node)
    }
    fn execute(&self, _argc: usize, _stack: &mut LinkedList<String>) -> Result<(), String> {
        panic!("procedure not implemented yet");
    }
}