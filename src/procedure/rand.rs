use std::cell::RefCell;
use std::rc::Rc;
use crate::procedure::Procedure;
use crate::program::Value;
use crate::vm::Stack;
use rand::RngExt;
use rand::rngs::SmallRng;
use rand::{SeedableRng};

pub struct Rand {
    rng: Rc<RefCell<SmallRng>>,
}

impl Rand {
    pub(crate) fn new() -> Rand {
        Rand {
            rng: Rc::new(RefCell::new(SmallRng::seed_from_u64(42)))
        }
    }
}

impl Procedure for Rand {
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        if argc != 0 {
            return Err(String::from("argument count must be zero"));
        }

        stack.push(Value::Float(self.rng.borrow_mut().random::<f64>()));

        Ok(())
    }
}