use crate::debug;
use crate::vm::{Program, Value, get_op_executable};

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
}

pub type Memo = Vec<Value>;

pub struct VM {}

impl VM {
    pub fn new() -> VM {
        VM {}
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

            debug!(println!("> {} {}", op, stack.len()));

            get_op_executable(&op.name)(pr, stack, memo);
        }
    }
}

#[cfg(debug_enabled)]
#[macro_export]
macro_rules! debug {
    ($body:expr) => {
        $body;
    };
}

#[cfg(not(debug_enabled))]
#[macro_export]
macro_rules! debug {
    ($body:expr) => {};
}
