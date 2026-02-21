use crate::compiler::Compiler;
use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::Procedure;

pub struct If {}

impl Procedure for If {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // IF (rand() > 1) (#MORE, #LESS)
        let expr = parser.subparse_one_in_bracers()?;

        let hash_links = parser.subparse_list_in_bracers(Some(2))?;

        if !hash_links.iter().all(Node::is_flow_link) {
            return Err("if must have a 2 flow link".to_string());
        }

        let mut params = vec![expr];
        params.extend(hash_links);

        Ok(Node::new_operation(token.value, params, token.at))
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        let expr = node.params.first().unwrap().clone();

        sc.sub_compile(expr).unwrap();

        sc.program.new_cskip(2);
        sc.program.new_jmp(node.params.get(2).unwrap().value.clone());
        sc.program.new_skip(1);
        sc.program.new_jmp(node.params.get(1).unwrap().value.clone());

        Ok(())
    }
}