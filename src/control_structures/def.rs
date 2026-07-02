use crate::compiler::Compiler;
use crate::control_structures::ControlStructures;
use crate::lexer::Token;
use crate::parser::{Node, Parser};

pub struct Def {}

impl ControlStructures for Def {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // DEF typename <<dynamic>>
        let define_entity = parser.subparse_word()?;

        return match define_entity.value.as_str() {
            "struct" => {
                subparse_struct(token, parser)
            },
            e => {
                Err(format!("unexpected entity {}", e))
            }
        }
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

fn subparse_struct(token: Token, parser: &mut Parser) -> Result<Node, String> {
    // DEF struct flip_coin_args {$TRESHOLD: float}
    let name = parser.subparse_word()?;
    let expr = parser.subparse_list_in_bracers(None)?;

    Ok(Node::new_struct_declaration(name.value.clone(), expr, token.at))
}