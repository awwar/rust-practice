use crate::procedure::Procedure;
use rand::RngExt;
use std::collections::LinkedList;

pub struct Rand {}

impl Procedure for Rand {
    fn execute(&self, argc: usize, stack: &mut LinkedList<String>) -> Result<(), String> {
        if argc != 0 {
            return Err(String::from("argument count must be zero"));
        }

        let mut rng = rand::rng();

        stack.push_front(rng.random::<f64>().to_string());

        Ok(())
    }
}