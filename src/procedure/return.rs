use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::Procedure;

struct Return {}

impl Procedure for Return {
    fn parse(token: Token, mut parser: Parser) -> Result<Node, String> {
        // RETURN (expression)
        let expr = match parser.subparse_one_in_bracers() {
            Ok(l) => l,
            Err(e) => return Err(e)
        };

        Ok(Node::new_operation(token.value, vec![expr], token.at))
    }
}