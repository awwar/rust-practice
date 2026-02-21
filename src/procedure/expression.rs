use crate::procedure::Procedure;
use crate::program::Value;
use crate::vm::Stack;

pub struct Expression {
    pub op: fn(l: &Value, r: &Value) -> Value,
}

impl Procedure for Expression {
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        if argc != 2 {
            panic!("Procedure expects 2 arguments");
        }

        let second_operand = stack.pop_back().unwrap();
        let first_operand = stack.pop_back().unwrap();

        let new_value = (self.op)(&first_operand, &second_operand);

        stack.push_front(new_value);

        Ok(())
    }
}
