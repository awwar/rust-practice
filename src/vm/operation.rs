use crate::procedure::get_procedures;
use crate::program::{Program, Value};
use crate::vm::vm::{Memo, Stack};
use std::collections::HashMap;

pub fn get_op_executable() -> HashMap<String, fn(&mut Program, &mut Stack, &mut Memo) -> ()> {
    let mut executables = HashMap::<String, fn(&mut Program, &mut Stack, &mut Memo) -> ()>::new();

    executables.insert("JMP".to_string(), |pr: &mut Program, _: &mut Stack, _: &mut Memo| {
        let mark_name = pr.current().unwrap().word.clone().unwrap();
        pr.trace_back();
        pr.jump_to_mark(mark_name);
    });

    executables.insert("EXEC".to_string(), |pr: &mut Program, st: &mut Stack, _: &mut Memo| {
        let op = pr.current().unwrap();

        let procedures = get_procedures();
        let proc_name = op.word.clone().unwrap();
        let proc = procedures.get(&proc_name.as_str()).unwrap();
        let argc = op.count.clone().unwrap();

        proc.execute(argc, st).unwrap();
    });

    executables.insert("MARK".to_string(), |pr: &mut Program, _: &mut Stack, _: &mut Memo| {
        pr.finish_block();
        pr.skip(0);
    });

    executables.insert("PUSH".to_string(), |pr: &mut Program, st: &mut Stack, mem: &mut Memo| {
        let op = pr.current().unwrap();

        let value = op.value.clone().unwrap().clone();
        let raw_val = value.clone().raw();

        if raw_val.starts_with("$") {
            st.push_front(mem[&raw_val].clone());
        } else {
            st.push_front(value);
        }
    });

    executables.insert("SKIP".to_string(), |pr: &mut Program, _: &mut Stack, _: &mut Memo| {
        let skip = pr.current().unwrap().count.unwrap().clone();

        pr.skip(skip);
    });

    executables.insert("CSKIP".to_string(), |pr: &mut Program, st: &mut Stack, _: &mut Memo| {
        let operand = st.pop_front().unwrap();

        let condition_result = operand.to_bool().eq(&Value::Boolean(true));

        match condition_result {
            Value::Boolean(true) => {
                let skip = pr.current().unwrap().count.unwrap().clone();

                pr.skip(skip);
            }
            _ => {}
        }
    });

    executables.insert("VAR".to_string(), |pr: &mut Program, st: &mut Stack, mem: &mut Memo| {
        let op = pr.current().unwrap();

        let var_name = op.word.clone().unwrap();

        let operand = st.pop_front().unwrap();

        if mem.contains_key(&var_name) {
            panic!("variable {} already defined", var_name);
        }

        mem.insert(var_name, operand);
    });

    return executables;
}