use crate::lexer::{Token, TokenStream};
use crate::operation::Procedure;
use crate::parser::{Node, Parser};

struct If {

}

impl Procedure for If {
    fn parse(token: Token, mut parser: Parser) -> Result<Node, String> {
        // IF (rand() > 1) (#MORE, #LESS)
        let expr = match parser.subparse_one_in_bracers() {
            Ok(l) => l,
            Err(e) => return Err(e)
        };

        let hash_links = match parser.subparse_list_in_bracers(Some(2)) {
            Ok(l) => l,
            Err(e) => return Err(e)
        };

        if !hash_links.iter().all(Node::is_flow_link) {
            return Err("if must have a 2 flow link".to_string());
        }

        Ok(Node::new_operation(token.value, hash_links.clone(), token.at))
    }
}