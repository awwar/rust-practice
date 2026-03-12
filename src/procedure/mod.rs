mod array;
mod expression;
mod print;
mod rand;
mod type_converter;

use crate::compiler::Compiler;
use crate::parser::Node;
use crate::vm::Stack;

pub trait Procedure {
    fn debug(&self) -> &str {
        std::any::type_name::<Self>()
    }
    fn support(&self, node: &Node) -> bool;
    fn parse(&self, node: Node) -> Result<Node, String> {
        Ok(node)
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        for child in &node.params {
            sc.sub_compile(child.clone())?;
        }

        let proc = get_procedures(&node);

        sc.program.new_exec(node.value.clone(), proc, node.params.len());

        Ok(())
    }
    fn execute(&self, _argc: usize, _stack: &mut Stack) -> Result<(), String>;
}

pub fn get_procedures(node: &Node) -> Box<dyn Procedure> {
    let mut procedures: Vec<Box<dyn Procedure>> = Vec::new();
    procedures.extend(array::get_procedures());
    procedures.extend(expression::get_procedures());
    procedures.extend(print::get_procedures());
    procedures.extend(rand::get_procedures());
    procedures.extend(type_converter::get_procedures());

    for child in procedures {
        if child.support(node) {
            return child;
        }
    }

    panic!("Unable to find procedure");
}
