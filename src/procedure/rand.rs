use std::cell::RefCell;
use std::rc::Rc;
use crate::procedure::{Procedure, Specification, Type};
use crate::vm::Value;
use crate::vm::Stack;
use rand::{Rng, RngExt};
use rand::rngs::SmallRng;
use rand::{SeedableRng};

pub fn get_procedures() -> Vec<Box<dyn Procedure>> {
    vec![
        Box::new(Rand::new()),
    ]
}

pub struct Rand {
    rng: Rc<RefCell<SmallRng>>,
}

impl Rand {
    pub(crate) fn new() -> Rand {
        let mut rng = rand::rng();

        Rand {
            rng: Rc::new(RefCell::new(SmallRng::seed_from_u64(rng.next_u64()))),
        }
    }
}

impl Procedure for Rand {
    fn spec(&self) -> Specification {
        Specification{
            method_name: "RAND",
            args: vec![],
            return_type: &Type::Float
        }
    }
    fn execute(&self, argc: usize, stack: &mut Stack) {
        assert_eq!(argc, 0, "Procedure expects 0 arguments");

        stack.push(Value::Float(self.rng.borrow_mut().random::<f64>()));
    }
}