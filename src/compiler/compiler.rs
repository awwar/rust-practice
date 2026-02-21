use crate::parser::{Node, NodeType};
use crate::procedure::get_procedures;
use crate::program::{Program, Value};

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

        if node_type == NodeType::Operation {
            let binding = get_procedures();
            let proc_name = node_copy.value.clone();
            let proc = binding.get(&proc_name.as_str());
            if proc.is_some() {
                let mut sub_compiler = Compiler::new();

                proc.unwrap().compile(&mut sub_compiler, node_copy.clone()).unwrap();

                self.program.merge(sub_compiler.program);

                return Ok(());
            }
        }

        self.sub_compile(node_copy)
    }

    pub fn sub_compile(&mut self, node: Node) -> Result<(), String> {
        let node_copy = node.clone();
        let node_type: NodeType = node.node_type;

        let mut from_param: usize = 0;
        if node_type == NodeType::FlowDeclaration {
            self.program.new_mark(node_copy.value.clone());

            for child in node_copy.params.iter() {
                from_param += 1;
                if child.node_type == NodeType::Constant {
                    break;
                }
                self.program.new_exec(child.value.clone(), 1);
                if child.params.len() != 1 {
                    return Err(format!("Invalid number of flow arguments: {}", child.params.len()));
                }
                self.program.new_var(child.params.get(0).unwrap().value.clone());
            }
        }

        for child in node_copy.params.iter().skip(from_param) {
            let child_copy = child.clone();
            match self.compile(child_copy) {
                Err(e) => return Err(e),
                _ => {}
            }
        }

        if node_type == NodeType::Variable {
            self.program.new_push(Value::String(node_copy.value.clone()));
        } else if node_type == NodeType::Operation {
            self.program.new_exec(node_copy.value.clone(), node_copy.params.len());
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
