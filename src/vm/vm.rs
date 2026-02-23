use crate::program::{Operation, Program, Value};
use crate::vm::operation::{get_op_executable};
use std::collections::BTreeMap;
use std::time::Duration;
use std::{env, thread};

pub struct Stack(Vec<Value>);

impl Stack {
    pub fn new() -> Stack {
        Stack(Vec::with_capacity(255))
    }
    pub fn push(&mut self, value: Value) {
        self.0.push(value);
    }
    pub fn pop(&mut self) -> Value {
        self.0.pop().unwrap()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub type Memo = BTreeMap<String, Value>;

pub struct VM {
    debug: bool,
}

impl VM {
    pub fn new() -> VM {
        let debug = env::var("DEBUG").unwrap_or_else(|_e| "0".to_string());

        VM {
            debug: debug.eq("1") || debug.eq("true")
        }
    }
    pub fn execute(&self, pr: &mut Program) {
        let stack = &mut Stack::new();
        let memo = &mut Memo::new();

        pr.jump_to_program_begin();

        loop {
            pr.next();

            if let Some(op) = pr.current() {
                self.debug(op, stack);

                get_op_executable(op.name)(pr, stack, memo);

                continue;
            }

            break;
        }
    }

    fn debug(&self, op: &Operation, stack: &Stack) {
        if !self.debug {
            return;
        }

        thread::sleep(Duration::from_millis(500));
        println!("> {} {}", op.to_string(), stack.len());
    }
}