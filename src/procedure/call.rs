use crate::compiler::Compiler;
use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::Procedure;

pub struct Call {}

impl Procedure for Call {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // CALL #NAME () $RESULT
        let link = parser.subparse_flow_link()?;

        let args = parser.subparse_list_in_bracers(None)?;

        let variable = parser.subparse_variable_name()?;

        let mut params = vec![link, variable];
        params.extend(args);

        Ok(Node::new_operation(token.value, params, token.at))
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        for n in node.params.iter().skip(2) {
            sc.sub_compile(n.clone()).unwrap();
        }

        sc.program.new_jmp(node.params.first().unwrap().value.clone());
        sc.program.new_var(node.params.get(1).unwrap().value.clone());

        Ok(())
    }
}
