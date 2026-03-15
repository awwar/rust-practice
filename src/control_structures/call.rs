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

        let mut node = Node::new_operation(procedure.value.to_uppercase(), args, token.at);

        if !node.is_flow_link_word() {
            let proc = get_procedures(&node.clone());

            node = proc.spec().parse(node)?;
        }

        Ok(Node::new_operation(
            token.value,
            vec![node, variable],
            token.at,
        ))
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        let subnode = node.params.first().unwrap();

        if subnode.is_flow_link_word() {
            for n in &subnode.params {
                sc.sub_compile(n.clone())?;
            }

            sc.program.new_jmp(subnode.value.clone());
        } else {
            sc.sub_compile(subnode.clone())?;
        }

        let var_name = node.params.last().unwrap().value.clone();

        if !var_name.eq("$_") {
            sc.program.new_heap(var_name);
        }

        Ok(())
    }
}
