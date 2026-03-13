use crate::parser::Node;
use crate::procedure::{Procedure, Specification, Type};
use crate::program::Value;
use crate::vm::Stack;

pub fn get_procedures() -> Vec<Box<dyn Procedure>> {
    vec![
        Box::new(Expression {v: "+",  op: Value::add }),
        Box::new(Expression {v: "-",  op: Value::subtract }),
        Box::new(Expression {v: "/",  op: Value::divide }),
        Box::new(Expression {v: "*",  op: Value::multiply }),
        Box::new(Expression {v: "^",  op: Value::power }),
        Box::new(Expression {v: "=",  op: Value::eq }),
        Box::new(Expression {v: "<",  op: Value::less }),
        Box::new(Expression {v: ">",  op: Value::more }),
    ]
}

pub struct Expression {
    op: fn(l: &Value, r: &Value) -> Value,
    v: &'static str,
}

impl Procedure for Expression {
    fn spec(&self) -> Specification {
        Specification{
            method_name: self.v,
            args: vec![Type::Any, Type::Any],
            return_type: Type::Any
        }
    }
    fn execute(&self, argc: usize, stack: &mut Stack) -> Result<(), String> {
        assert_eq!(argc, 2, "Procedure expects 2 arguments");

        let second_operand = stack.pop();
        let first_operand = stack.pop();

        let new_value = (self.op)(&first_operand, &second_operand);

        stack.push(new_value);

        Ok(())
    }
}
