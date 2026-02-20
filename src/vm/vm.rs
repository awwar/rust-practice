use crate::program::{Operation, Program};
use crate::vm::operation::get_op_executable;
use std::collections::{HashMap, LinkedList};
use std::time::Duration;
use std::{env, thread};

pub(crate) type Stack = LinkedList<String>;
pub(crate) type Memo = HashMap<String, String>;

pub fn execute(pr: &mut Program) {
    let stack = &mut Stack::new();
    let memo = &mut Memo::new();

    pr.jump_to_program_begin();

    let op_executable_map = get_op_executable();
    loop {
        pr.next();

        let op = match pr.current() {
            None => break,
            Some(o) => o,
        };

        let op_executable = match op_executable_map.get(op.name) {
            None => panic!("unknown procedure: {}", op.name),
            Some(p) => p,
        };

        debug(op, stack);

        op_executable(pr, stack, memo);
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