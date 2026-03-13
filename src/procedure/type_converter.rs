use crate::parser::Node;
use crate::procedure::{Procedure, Specification, Type};
use crate::program::Value;
use crate::vm::Stack;

pub fn get_procedures() -> Vec<Box<dyn Procedure>> {
    vec![
        Box::new(TypeConverter {v: "BOOL", op: Value::to_bool }),
        Box::new(TypeConverter {v: "FLOAT", op: Value::to_float }),
        Box::new(TypeConverter {v: "STRING", op: Value::to_string }),
        Box::new(TypeConverter {v: "INT", op: Value::to_integer }),
        Box::new(TypeConverter {v: "ARRAY", op: |_| Value::Array(Vec::<Value>::new()) }),
        Box::new(TypeConverter {v: "VOID", op: |_| Value::Integer(0) }),
    ]
}

pub struct TypeConverter {
    op: fn(l: &Value) -> Value,
    v: &'static str,
}

impl Procedure for TypeConverter {
    fn spec(&self) -> Specification {
        Specification{
            method_name: self.v,
            args: vec![Type::Any],
            return_type: Type::Any
        }
    }
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        assert_eq!(argc, 1, "Procedure expects 1 arguments");

        let first_operand = stack.pop();

        let new_value = (self.op)(&first_operand);

        stack.push(new_value);

        Ok(())
    }
}