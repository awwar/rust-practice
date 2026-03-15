use crate::compiler::Compiler;
use crate::control_structures::ControlStructures;
use crate::lexer::Token;
use crate::parser::{Node, Parser};

pub struct Return {}

impl ControlStructures for Return {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // RETURN (expression)
        let expr = parser.subparse_one_in_bracers()?;

        Ok(Node::new_operation(token.value, vec![expr], token.at))
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        let child = node.params.first().unwrap();

        sc.sub_compile(child.clone())
    }
}
