use crate::compiler::Compiler;
use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::Procedure;

pub struct Return {}

impl Procedure for Return {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // RETURN (expression)
        let expr = match parser.subparse_one_in_bracers() {
            Ok(l) => l,
            Err(e) => return Err(e)
        };

        Ok(Node::new_operation(token.value, vec![expr], token.at))
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        let child = node.params.get(0).unwrap();

        sc.sub_compile(child.clone())
    }
}