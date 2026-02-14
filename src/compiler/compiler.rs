use crate::parser::Node;

pub trait SubCompiler {
    fn compile(node: Node) -> Option<String>;
}