use crate::program::{Operation, Program, Value};
use crate::vm::operation::get_op_executable;
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

pub type Memo = Vec<Value>;

pub struct VM {
    debug: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            debug: env::var("DEBUG")
                .unwrap_or_else(|_e| "0".to_string())
                .eq("1"),
        }
    }
    pub fn execute(&self, pr: &mut Program) {
        let stack = &mut Stack::new();
        let memo = &mut Memo::with_capacity(pr.get_memo_size());
        memo.resize(pr.get_memo_size(), Value::Null);

        pr.jump_to_program_begin();

        loop {
            if !pr.next() {
                break;
            }

            let op = pr.current();

            self.debug(op, stack);

            get_op_executable(&op.name)(pr, stack, memo);
        }
    }

    fn debug(&self, op: &Operation, stack: &Stack) {
        if !self.debug {
            return;
        }

        thread::sleep(Duration::from_millis(200));
        println!("> {} {}", op.to_string(), stack.len());
    }
}
