use crate::parser::Node;

pub struct Compiler {}

pub trait SubCompiler {
    fn compile(&self, node: Node) -> Option<String>;
}

impl SubCompiler for Compiler {
    fn compile(&self, _node: Node) -> Option<String> {
        None
    }
}