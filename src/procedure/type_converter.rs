use crate::procedure::Procedure;
use std::collections::LinkedList;

pub struct TypeConverter {}

impl Procedure for TypeConverter {
    fn execute(&self, _argc: usize, _stack: &mut LinkedList<String>) -> Result<(), String> {
        Ok(())
    }
}