use crate::procedure::Procedure;
use std::collections::LinkedList;

pub struct Expression {
    pub op: fn(l: String, r: String) -> String,
}

impl Procedure for Expression {
    fn execute(&self, argc: usize, stack: &mut LinkedList<String>) -> Result<(), String> {
        if argc != 2 {
            panic!("Procedure expects 2 arguments");
        }

        let second_operand = stack.pop_back().unwrap();
        let first_operand = stack.pop_back().unwrap();

        let new_value = (self.op)(first_operand, second_operand);

        stack.push_front(new_value);

        return Ok(());
    }
}
