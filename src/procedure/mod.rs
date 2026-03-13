mod array;
mod expression;
mod print;
mod rand;
mod type_converter;

use crate::compiler::Compiler;
use crate::parser::Node;
use crate::vm::Stack;

pub enum Type {
    Array(Box<Type>),
    Integer,
    Float,
    String,
    Bool,
    Null,
    None,
    Any,
}

impl Type {
    pub fn repr(&self) -> String {
        match self {
            Type::Array(tp) => format!("<array of {}>", tp.repr()),
            Type::Integer => "<integer>".to_string(),
            Type::Float => "<float>".to_string(),
            Type::String => "<string>".to_string(),
            Type::Bool => "<bool>".to_string(),
            Type::Null => "<null>".to_string(),
            Type::None => "<none>".to_string(),
            Type::Any => "<any>".to_string(),
        }
    }
}

pub struct Specification {
    method_name: &'static str,
    args: Vec<Type>,
    return_type: Type,
}

impl Specification {
    pub fn support(&self, node: &Node) -> bool {
        node.value
            .to_uppercase()
            .eq(self.method_name.to_uppercase().as_str())
            && node.params.len() == self.args.len()
    }
    pub fn debug(&self) -> String {
        format!(
            "{}({}): {}",
            self.method_name,
            self.args
                .iter()
                .map(Type::repr)
                .collect::<Vec<_>>()
                .join(", "),
            self.return_type.repr(),
        )
    }
    pub fn parse(&self, node: Node) -> Result<Node, String> {
        Ok(node)
    }
}

pub trait Procedure {
    fn spec(&self) -> Specification;
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        for child in &node.params {
            sc.sub_compile(child.clone())?;
        }

        let proc = get_procedures(&node);

        sc.program.new_exec(proc, node.params.len());

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
        if child.spec().support(node) {
            return child;
        }
    }

    panic!("Unable to find procedure");
}
