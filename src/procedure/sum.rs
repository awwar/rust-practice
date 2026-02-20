use crate::procedure::Procedure;
use std::collections::LinkedList;

pub struct Sum {}

impl Procedure for Sum {
    fn execute(&self, argc: usize, stack: &mut LinkedList<String>) -> Result<(), String> {
        if argc == 0 {
            return Ok(());
        }

        let mut result: String = stack.pop_front().unwrap();

        for _ in 1..argc {
            let operand = stack.pop_front().unwrap();

            result = format!("{}{}", operand, result);
        }

        stack.push_front(result);

        Ok(())
    }
}