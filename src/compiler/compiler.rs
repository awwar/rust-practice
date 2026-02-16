use crate::parser::{Node, NodeType};
use crate::procedure::PROCEDURES;
use crate::program::Program;

pub struct Compiler {
    pub program: Program,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler { program: Program::new() }
    }
    pub fn compile(&mut self, node: Node) -> Result<(), String> {
        let node_copy = node.clone();
        let node_type = node.node_type;

        if node_type == NodeType::FlowDeclaration {
            self.program.new_mark(node.value);

            let mut i = 0;
            let mut argc = 0;
            for child in node.params.iter() {
                if i == 0 {
                    //ToDo redo when return type will support
                } else if i == 1 {
                    argc = child.value.parse::<i32>().unwrap()
                } else if i < argc + 2 {
                    self.program.new_var(child.value.clone());
                } else {
                    self.program.new_exec(child.value.clone(), child.params.len());
                }
                i += 1
            }

            return Ok(());
        }

        for child in node.params.iter() {
            match self.compile(child.clone()) {
                Err(e) => return Err(e),
                _ => {}
            }
        }

        if node_type == NodeType::Variable {
            self.program.new_push(node.value);
        } else if node_type == NodeType::Operation {
            for proc in PROCEDURES {
                if proc.0.eq(&node.value.to_uppercase()) {
                    let mut sub_compiler = Compiler::new();

                    proc.1.compile(&mut sub_compiler, node_copy.clone()).unwrap();

                    self.program.merge(sub_compiler.program);

                    return Ok(());
                }
            }

            self.program.new_exec(node.value, node.params.len());
        } else if node_type == NodeType::Constant {
            self.program.new_push(node.value);
        }

        Ok(())
    }
}
