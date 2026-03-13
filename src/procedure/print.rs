use crate::procedure::{Procedure, Specification, Type};
use crate::program::Value;
use crate::vm::Stack;

pub fn get_procedures() -> Vec<Box<dyn Procedure>> {
    vec![
        Box::new(Print {}),
    ]
}

pub struct Print {}

impl Procedure for Print {
    fn spec(&self) -> Specification {
        Specification{
            method_name: "PRINT",
            args: vec![Type::String],
            return_type: Type::None
        }
    }
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        if argc != 1 {
            return Err(String::from("argument count must be 1"));
        }

        let _first_operand = stack.pop();
        // println!("{}", _first_operand.repr());

        Ok(())
    }
}