use crate::control_structures::get_control_structures;
use crate::parser::{Node, NodeType};
use crate::procedure::get_procedures;
use crate::vm::{Program, Value};

pub struct Compiler {
    pub program: Program,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            program: Program::new(),
        }
    }
    pub fn compile(&mut self, node: Node) -> Result<(), String> {
        let node_copy = node.clone();
        let node_type: NodeType = node.node_type;

        if node_type == NodeType::Program {
            for child in &node_copy.params {
                let child_copy = child.clone();
                self.compile(child_copy)?;
            }

            return Ok(());
        }

        if node_type == NodeType::FlowDeclaration {
            let mut from_param: usize = 0;

            self.program.new_mark(node_copy.value.clone());

            for child in &node_copy.params {
                from_param += 1;
                if child.node_type == NodeType::Constant {
                    break;
                }
                self.sub_compile(child.params.first().unwrap().clone())?;
                self.program.new_heap(child.params.last().unwrap().value.clone());
            }

            for child in node_copy.params.iter().skip(from_param) {
                let child_copy = child.clone();
                self.compile(child_copy)?;
            }

            return Ok(());
        }

        if node_type == NodeType::Operation {
            let proc_name = node_copy.value.as_str();
            let procedure = get_control_structures(proc_name);

            procedure.compile(self, node_copy.clone())?;

            return Ok(());
        }

        Err(format!("Invalid node type: {node_type:?}"))
    }

    pub fn sub_compile(&mut self, node: Node) -> Result<(), String> {
        let node_copy = node.clone();
        let node_type: NodeType = node.node_type;

        if node_type == NodeType::Operation {
            let procedure = get_procedures(&node_copy);

            procedure.compile(self, node_copy.clone())?;
        } else if node_type == NodeType::Variable {
            self.program.new_push_var(node_copy.value.clone());
        } else if node_type == NodeType::Constant || node_type == NodeType::String {
            self.program.new_push(Value::String(node_copy.value.clone()));
        } else if node_type == NodeType::Float {
            self.program.new_push(Value::Float(node_copy.value.parse::<f64>().unwrap()));
        } else if node_type == NodeType::Integer {
            self.program.new_push(Value::Integer(node_copy.value.parse::<i64>().unwrap()));
        }

        Ok(())
    }
}
