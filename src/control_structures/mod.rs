use crate::compiler::Compiler;
use crate::lexer::Token;
use crate::parser::{Node, Parser};

pub mod call;
pub mod cond;
pub mod ret;
pub mod var;

pub trait ControlStructures {
    fn parse(&self, token: Token, _parser: &mut Parser) -> Result<Node, String>;
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String>;
}

pub fn get_control_structures(name: &str) -> Box<dyn ControlStructures> {
    match name {
        "CALL" => Box::new(call::Call {}),
        "IF" => Box::new(cond::If {}),
        "RETURN" => Box::new(ret::Return {}),
        "VAR" => Box::new(var::Var {}),
        _ => panic!("Unknown control structure {name}"),
    }
}
