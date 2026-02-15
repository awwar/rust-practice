use crate::compiler::SubCompiler;
use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::program::{Program, ValueConverter};
use std::collections::LinkedList;

pub trait Procedure {
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

pub struct ProcedureItem {
    procedure: Box<dyn Procedure>,
    name: String,
}

impl ProcedureItem {
    pub fn new(procedure: Box<dyn Procedure>, name: String) -> ProcedureItem {
        ProcedureItem { procedure, name }
    }
}

inventory::collect!(ProcedureItem);