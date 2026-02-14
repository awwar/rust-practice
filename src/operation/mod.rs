mod call;
mod r#if;

use std::collections::LinkedList;
use crate::lexer::Token;
use crate::parser::node::Node;
use crate::parser::Parser;
use crate::program::{Program, ValueConverter};
use crate::compiler::{SubCompiler};

trait Procedure {
    fn parse(token: Token, parser: Parser) -> Result<Node, String> {
        Ok(Node::new_operation(token.value, vec![], token.at))
    }
    fn compile(prog: Program, node: Node, sub_compiler: impl SubCompiler) -> Option<String> {
        sub_compiler.compile(node)
    }
    fn execute(argc: usize, stack: LinkedList<Box<dyn ValueConverter>>) -> Option<String> {
        panic!("procedure not implemented yet");
    }
}