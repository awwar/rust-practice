use crate::procedure::Procedure;
use crate::program::Value;
use crate::vm::Stack;

pub struct TypeConverter {
    pub op: fn(l: &Value) -> Value,
    pub opname: String,
}

impl Procedure for TypeConverter {
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        if argc != 1 {
            panic!("Procedure expects 1 arguments");
        }

        let first_operand = stack.pop_back().unwrap();

        let new_value = (self.op)(&first_operand);

        stack.push_front(new_value);

        Ok(())
    }
}