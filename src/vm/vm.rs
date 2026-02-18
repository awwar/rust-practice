use crate::program::Program;
use std::collections::{HashMap, LinkedList};

type Stack = LinkedList<String>;
type Memo = HashMap<String, String>;

pub fn execute(mut pr: &Program) {
    let mut stack = &Stack::new();
    let mut memo = &Memo::new();

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

        op_executable(pr, stack, memo)
    }
}

fn get_op_executable() -> HashMap<String, fn(&Program, &Stack, &Memo) -> ()> {
    let mut procedures = HashMap::<String, fn(&Program, &Stack, &Memo) -> ()>::new();

    procedures.insert("JMP".to_string(), |mut pr: &Program, st: &Stack, mem: &Memo| {
        let op = pr.current().unwrap().clone();
        pr.trace_back();
        pr.jump_to_mark(op.word.clone());
    });

    return procedures;
}