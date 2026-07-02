use crate::compiler::Compiler;
use crate::control_structures::ControlStructures;
use crate::lexer::Token;
use crate::parser::{Node, Parser};

pub struct Var {}

impl ControlStructures for Var {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // VAR $VAR_NAME (expression)
        let variable_name = parser.subparse_variable_name()?;
                
        let expr = parser.subparse_one_in_bracers()?;

        Ok(Node::new_operation(
            token.value,
            vec![variable_name, expr],
            token.at,
        ))
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        let var_name = node.params.first().unwrap().value.clone();

        if !var_name.eq("$_") {
            sc.sub_compile(node.params.last().unwrap().clone())?;

            sc.program.new_heap(var_name);
        }

        Ok(())
    }
}
