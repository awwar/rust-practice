use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::procedure::ProcedureItem;
use crate::procedure::Procedure;

struct If {}

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

        let params = vec![expr].into_iter().chain(hash_links.into_iter()).collect::<Vec<Node>>();

        Ok(Node::new_operation(token.value, params, token.at))
    }
}

inventory::submit! {
    ProcedureItem::new(Box::new(If{}), "IF".to_owned())
}