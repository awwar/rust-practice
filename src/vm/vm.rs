use crate::program::{Operation, Program, Value};
use crate::vm::operation::{get_op_executable};
use std::collections::{BTreeMap, HashMap};
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
}

impl VM {
    pub fn new() -> VM {
        VM {
        }
    }
    pub fn execute(&self, pr: &mut Program) {
        let stack = &mut Stack::new();
        let memo = &mut Memo::new();

        pr.jump_to_program_begin();

        loop {
            pr.next();

            let op = match pr.current() {
                None => break,
                Some(o) => o,
            };

            get_op_executable(op.name, pr, stack, memo);
        }
    }
}



fn debug(op: &Operation, stack: &Stack) {
    let debug = match env::var("DEBUG") {
        Ok(val) => val,
        Err(_e) => return,
    };

    if debug.ne("1") && debug.ne("true") {
        return;
    }

    thread::sleep(Duration::from_millis(500));
    println!("> {} {}", op.to_string(), stack.len());
}