use crate::compiler::Compiler;
use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::Procedure;

pub struct Var {}

impl Procedure for Var {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // RETURN (expression)
        let expr = match parser.subparse_one_in_bracers() {
            Ok(l) => l,
            Err(e) => return Err(e)
        };

        let variable_name = match parser.subparse_variable_name() {
            Ok(l) => l,
            Err(e) => return Err(e)
        };

        Ok(Node::new_operation(token.value, vec![variable_name, expr], token.at))
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        sc.sub_compile(node.params.get(1).unwrap().clone()).unwrap();
        sc.program.new_var(node.params.get(0).unwrap().value.clone());

        Ok(())
    }
}