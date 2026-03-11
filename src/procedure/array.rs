use crate::procedure::Procedure;
use crate::program::Value;
use crate::vm::Stack;
use rand::prelude::SmallRng;
use rand::{Rng, RngExt, SeedableRng};
use std::cell::RefCell;
use std::rc::Rc;
use crate::parser::Node;

pub fn get_procedures() -> Vec<Box<dyn Procedure>> {
    vec![
        Box::new(FillRandom::new()),
        Box::new(At {}),
    ]
}

pub struct FillRandom {
    rng: Rc<RefCell<SmallRng>>,
}

impl FillRandom {
    pub(crate) fn new() -> FillRandom {
        let mut rng = rand::rng();

        FillRandom {
            rng: Rc::new(RefCell::new(SmallRng::seed_from_u64(rng.next_u64()))),
        }
    }
}

// FILL_RANDOM ($INITIAL_VALUE, 10, -100, 100) $FILLED_VALUE
impl Procedure for FillRandom {
    fn support(&self, node: &Node) -> bool {
        if node.params.len() != 4 {
            return false;
        }

        return true;
    }
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        if argc != 4 {
            return Err(String::from("argument count must be 4"));
        }

        let Value::Integer(max) = stack.pop() else { todo!() };
        let Value::Integer(min) = stack.pop() else { todo!() };
        let Value::Integer(size) = stack.pop() else { todo!() };
        let Value::Array(array) = stack.pop() else { todo!() };

        let mut new_val = array.clone();
        let addition = (0..size).map(|_| Value::Integer(self.rng.borrow_mut().random_range(min..max))).collect::<Vec<Value>>();
        new_val.extend(addition);

        stack.push(Value::Array(new_val));

        Ok(())
    }
}

pub struct At {}

impl Procedure for At {
    fn support(&self, node: &Node) -> bool {
        return true;
    }

    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        if argc != 2 {
            return Err(String::from("argument count must be 2"));
        }

        let Value::Integer(count) = stack.pop() else { todo!() };
        let Value::Array(array) = stack.pop() else { todo!() };
        let val: Value = (array)[count as usize].clone();

        stack.push(val);

        Ok(())
    }
}
