use crate::parser::Node;
use crate::procedure::Procedure;
use crate::vm::Stack;

pub fn get_procedures() -> Vec<Box<dyn Procedure>> {
    vec![
        Box::new(Print {}),
    ]
}

pub struct Print {}

impl Procedure for Print {
    fn support(&self, node: &Node) -> bool {
        node.value.eq("PRINT")
    }

    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        if argc != 1 {
            return Err(String::from("argument count must be 1"));
        }

        let _first_operand = stack.pop();
        // println!("{}", first_operand.repr());

        Ok(())
    }
}