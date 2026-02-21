use crate::procedure::Procedure;
use crate::program::Value;
use crate::vm::Stack;
use rand::RngExt;

pub struct Rand {}

impl Procedure for Rand {
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        if argc != 0 {
            return Err(String::from("argument count must be zero"));
        }

        let mut rng = rand::rng();

        stack.push_front(Value::Float(rng.random::<f64>()));

        Ok(())
    }
}