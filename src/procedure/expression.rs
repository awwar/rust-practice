use crate::procedure::Procedure;
use crate::program::Value;
use crate::vm::Stack;

pub struct Expression {
    pub op: fn(l: &Value, r: &Value) -> Value,
}

impl Procedure for Expression {
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        assert_eq!(argc, 2, "Procedure expects 2 arguments");

        let second_operand = stack.pop();
        let first_operand = stack.pop();

        let new_value = (self.op)(&first_operand, &second_operand);

        stack.push(new_value);

        Ok(())
    }
}
