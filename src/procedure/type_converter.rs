use crate::procedure::Procedure;
use crate::program::Value;
use crate::vm::Stack;

pub struct TypeConverter {
    pub op: fn(l: &Value) -> Value,
}

impl Procedure for TypeConverter {
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        assert_eq!(argc, 1, "Procedure expects 1 arguments");

        let first_operand = stack.pop();

        let new_value = (self.op)(&first_operand);

        stack.push(new_value);

        Ok(())
    }
}