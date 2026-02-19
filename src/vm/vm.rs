use crate::program::Program;
use std::collections::{HashMap, LinkedList};
use std::thread;
use std::time::Duration;
use crate::procedure::get_procedures;

type Stack = LinkedList<String>;
type Memo = HashMap<String, String>;

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

        thread::sleep(Duration::from_secs(1));
        println!("> {} {}", op.clone().to_string(), stack.len());

        op_executable(pr, stack, memo);
    }
}

fn get_op_executable() -> HashMap<String, fn(&mut Program, &mut Stack, &mut Memo) -> ()> {
    let mut procedures = HashMap::<String, fn(&mut Program, &mut Stack, &mut Memo) -> ()>::new();

    procedures.insert("JMP".to_string(), |pr: &mut Program, st: &mut Stack, mem: &mut Memo| {
        let mark_name = pr.current().unwrap().word.clone();
        pr.trace_back();
        pr.jump_to_mark(mark_name);
    });

    procedures.insert("EXEC".to_string(), |pr: &mut Program, st: &mut Stack, mem: &mut Memo| {
        let op = pr.current().unwrap();

        let procedures = get_procedures();
        let proc = procedures.get(&op.word.to_uppercase()).unwrap();
        let argc = op.count.clone();

        proc.execute(argc, st).unwrap();
    });

    procedures.insert("MARK".to_string(), |pr: &mut Program, st: &mut Stack, mem: &mut Memo| {
        pr.finish_block();
        pr.skip(0);
    });

    procedures.insert("PUSH".to_string(), |pr: &mut Program, st: &mut Stack, mem: &mut Memo| {
        let op = pr.current().unwrap();

        let value = op.word.clone();

        if value.starts_with("$") {
            st.push_front(mem[&value].clone());
        } else {
            st.push_front(value);
        }
    });

    procedures.insert("SKIP".to_string(), |pr: &mut Program, st: &mut Stack, mem: &mut Memo| {
        let skip = pr.current().unwrap().count.clone();

        pr.skip(skip);
    });

    procedures.insert("CSKIP".to_string(), |pr: &mut Program, st: &mut Stack, mem: &mut Memo| {
        let operand = st.pop_front().unwrap();

        if operand != "true" {
            let skip = pr.current().unwrap().count.clone();

            pr.skip(skip);
        }
    });

    procedures.insert("VAR".to_string(), |pr: &mut Program, st: &mut Stack, mem: &mut Memo| {
        let op = pr.current().unwrap();

        let var_name = op.word.clone();

        let operand = st.pop_front().unwrap();

        if mem.contains_key(&var_name) {
            panic!("variable {} already defined", var_name);
        }

        mem.insert(var_name, operand);
    });

    return procedures;
}