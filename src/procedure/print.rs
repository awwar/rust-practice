use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::Procedure;

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
}