use crate::procedure::Procedure;
use crate::program::Value;
use crate::vm::Stack;

pub struct Sum {}

impl Procedure for Sum {
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        if argc == 0 {
            return Ok(());
        }

        let mut result: Value = stack.pop_front().unwrap();

        for _ in 1..argc {
            let operand = stack.pop_front().unwrap();

            result = operand.add(&result);
        }

        stack.push_front(result);

        Ok(())
    }
}