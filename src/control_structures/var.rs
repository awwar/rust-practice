use crate::compiler::Compiler;
use crate::control_structures::ControlStructures;
use crate::lexer::Token;
use crate::parser::{Node, Parser};

pub struct Var {}

impl ControlStructures for Var {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // VAR (expression) $VAR_NAME
        let expr = parser.subparse_one_in_bracers()?;

        let variable_name = parser.subparse_variable_name()?;

        Ok(Node::new_operation(token.value, vec![variable_name, expr], token.at))
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        sc.sub_compile(node.params.get(1).unwrap().clone())?;

        let var_name = node.params.first().unwrap().value.clone();

        if !var_name.eq("$_") {
            sc.program.new_var(var_name);
        }

        Ok(())
    }
}