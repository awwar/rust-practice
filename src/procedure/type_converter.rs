use crate::procedure::{Procedure, Specification, Type};
use crate::program::Value;
use crate::vm::Stack;

pub fn get_procedures() -> Vec<Box<dyn Procedure>> {
    vec![
        TypeConverter::new("BOOL", &Type::Integer, &Type::Bool, Value::to_bool),
        TypeConverter::new("BOOL", &Type::Float, &Type::Bool, Value::to_bool),
        TypeConverter::new("BOOL", &Type::Bool, &Type::Bool, Value::to_bool),
        TypeConverter::new("BOOL", &Type::String, &Type::Bool, Value::to_bool),

        TypeConverter::new("FLOAT", &Type::Integer, &Type::Float, Value::to_float),
        TypeConverter::new("FLOAT", &Type::Float, &Type::Float, Value::to_float),
        TypeConverter::new("FLOAT", &Type::Bool, &Type::Float, Value::to_float),
        TypeConverter::new("FLOAT", &Type::String, &Type::Float, Value::to_float),

        TypeConverter::new("STRING", &Type::Integer, &Type::String, Value::to_string),
        TypeConverter::new("STRING", &Type::Float, &Type::String, Value::to_string),
        TypeConverter::new("STRING", &Type::Bool, &Type::String, Value::to_string),
        TypeConverter::new("STRING", &Type::String, &Type::String, Value::to_string),

        TypeConverter::new("INTEGER", &Type::Integer, &Type::Integer, Value::to_integer),
        TypeConverter::new("INTEGER", &Type::Float, &Type::Integer, Value::to_integer),
        TypeConverter::new("INTEGER", &Type::Bool, &Type::Integer, Value::to_integer),
        TypeConverter::new("INTEGER", &Type::String, &Type::Integer, Value::to_integer),

        TypeConverter::new("ARRAY", &Type::Any(1), &Type::Array(&Type::Any(1)), |_: &Value| Value::Array(Vec::<Value>::new())),
        TypeConverter::new("VOID", &Type::Any(1), &Type::None, |_: &Value| Value::Null),
    ]
}

pub struct TypeConverter {
    v: &'static str,
    i: &'static Type,
    r: &'static Type,
    op: fn(l: &Value) -> Value,
}

impl TypeConverter {
    pub fn new(
        v: &'static str,
        i: &'static Type,
        r: &'static Type,
        op: fn(l: &Value) -> Value,
    ) -> Box<TypeConverter> {
        Box::new(TypeConverter { v, i, r, op })
    }
}

impl Procedure for TypeConverter {
    fn spec(&self) -> Specification {
        Specification {
            method_name: self.v,
            args: vec![self.i],
            return_type: self.r,
        }
    }
    fn execute(&self, argc: usize, stack: &mut Stack) {
        assert_eq!(argc, 1, "Procedure expects 1 arguments");

        let first_operand = stack.pop();

        let new_value = (self.op)(&first_operand);

        stack.push(new_value);
    }
}
