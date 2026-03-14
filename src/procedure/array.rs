use crate::procedure::{Procedure, Specification, Type};
use crate::program::Value;
use crate::vm::Stack;
use rand::prelude::SmallRng;
use rand::{Rng, RngExt, SeedableRng};
use std::cell::RefCell;
use std::rc::Rc;

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
    fn spec(&self) -> Specification {
        Specification{
            method_name: "FILL_RANDOM",
            args: vec![&Type::Array(&Type::Integer), &Type::Integer, &Type::Integer, &Type::Integer],
            return_type: &Type::Array(&Type::Integer)
        }
    }
    fn execute(&self, argc: usize, stack: &mut Stack) {
        assert_eq!(argc, 4, "Procedure expects 4 arguments");

        let Value::Integer(max) = stack.pop() else { todo!() };
        let Value::Integer(min) = stack.pop() else { todo!() };
        let Value::Integer(size) = stack.pop() else { todo!() };
        let Value::Array(array) = stack.pop() else { todo!() };

        let mut new_val = array.clone();
        let addition = (0..size).map(|_| Value::Integer(self.rng.borrow_mut().random_range(min..max))).collect::<Vec<Value>>();
        new_val.extend(addition);

        stack.push(Value::Array(new_val));
    }
}

pub struct At {}

impl Procedure for At {
    fn spec(&self) -> Specification {
        Specification{
            method_name: "AT",
            args: vec![&Type::Array(&Type::Any(1)), &Type::Integer],
            return_type: &Type::Any(1)
        }
    }
    fn execute(&self, argc: usize, stack: &mut Stack) {
        assert_eq!(argc, 2, "Procedure expects 2 arguments");

        let Value::Integer(count) = stack.pop() else { todo!() };
        let Value::Array(array) = stack.pop() else { todo!() };
        let val: Value = (array)[count as usize].clone();

        stack.push(val);
    }
}
