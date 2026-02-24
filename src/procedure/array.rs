use crate::lexer::Token;
use crate::parser::{Node, NodeType, Parser};
use crate::procedure::Procedure;
use crate::program::Value;
use crate::vm::Stack;
use rand::prelude::SmallRng;
use rand::{Rng, RngExt, SeedableRng};
use std::cell::RefCell;
use std::rc::Rc;
use crate::compiler::Compiler;

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

impl Procedure for FillRandom {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // FILL_RANDOM ($INITIAL_VALUE, 10, -100, 100) $FILLED_VALUE
        let mut exprs = parser.subparse_list_in_bracers(Some(4))?;

        assert_eq!(exprs[0].node_type, NodeType::Variable);
        assert_eq!(exprs[1].node_type, NodeType::Integer);
        assert_eq!(exprs[2].node_type, NodeType::Integer);
        assert_eq!(exprs[3].node_type, NodeType::Integer);

        let variable_name = parser.subparse_variable_name()?;

        exprs.extend(vec![variable_name]);

        Ok(Node::new_operation(token.value, exprs, token.at))
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        for param in node.params.iter().take(node.params.len() - 1) {
            sc.sub_compile(param.clone()).unwrap();
        }

        sc.program.new_exec(node.value.to_string(), 4);
        sc.program.new_var(node.params.last().unwrap().value.clone());

        Ok(())
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
