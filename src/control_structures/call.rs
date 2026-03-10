use crate::compiler::Compiler;
use crate::control_structures::ControlStructures;
use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::get_procedures;

pub struct Call {}

impl ControlStructures for Call {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // CALL #NAME () $RESULT
        let procedure = parser.subparse_word()?;

        let args = parser.subparse_list_in_bracers(None)?;

        let variable = parser.subparse_variable_name()?;

        if procedure.is_flow_link() {
            let mut params = vec![variable];
            params.extend(args);

            let node = Node::new_operation(procedure.value, params, token.at);

            Ok(Node::new_operation(token.value, vec![node], token.at))
        } else {
            let proc = get_procedures(procedure.value.to_uppercase().as_str());

            let node = proc.parse(procedure, variable, args)?;

            Ok(Node::new_operation(token.value, vec![node], token.at))
        }
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        let subnode = node.params.first().unwrap();

        for n in subnode.params.iter().skip(1) {
            sc.sub_compile(n.clone())?;
        }

        if subnode.is_flow_link() {
            sc.program.new_jmp(subnode.value.clone());
        } else {
            sc.program.new_exec(subnode.value.clone(), subnode.params.len() - 1);
        }

        let var_name = subnode.params.first().unwrap().value.clone();

        if !var_name.eq("$_") {
            sc.program.new_var(var_name);
        }

        Ok(())
    }
}
