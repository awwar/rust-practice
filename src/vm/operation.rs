use crate::program::{Program, Value};
use crate::vm::vm::{Memo, Stack};
use std::collections::HashMap;
use crate::procedure::Procedure;

pub type Executable = fn(&mut Program, &mut Stack, &mut Memo);

pub fn jmp(pr: &mut Program, _: &mut Stack, _: &mut Memo) {
    let mark_name = pr.current().unwrap().word.clone().unwrap();
    pr.trace_back();
    pr.jump_to_mark(mark_name);
}

pub fn exec(pr: &mut Program, st: &mut Stack, _: &mut Memo) {
    let op = pr.current().unwrap();

    let binding = op.word.clone().unwrap();
    let proc = pr.get_procedures().get(&binding.as_str()).unwrap();
    let argc = op.count.unwrap();

    proc.execute(argc, st).unwrap();
}

pub fn mark(pr: &mut Program, _: &mut Stack, _: &mut Memo) {
    pr.finish_block();
    pr.skip(0);
}

pub fn push(pr: &mut Program, st: &mut Stack, mem: &mut Memo) {
    let op = pr.current().unwrap();

    let value = op.value.clone().unwrap().clone();
    let raw_val = value.clone().raw();

    if raw_val.starts_with('$') {
        st.push(mem[&raw_val].clone());
    } else {
        st.push(value);
    }
}

pub fn skip(pr: &mut Program, _: &mut Stack, _: &mut Memo) {
    let skip = pr.current().unwrap().count.unwrap();

    pr.skip(skip);
}

pub fn cskip(pr: &mut Program, st: &mut Stack, _: &mut Memo) {
    let operand = st.pop();

    let condition_result = operand.to_bool().eq(&Value::Boolean(true));

    if let Value::Boolean(true) = condition_result {
        let skip = pr.current().unwrap().count.unwrap();

        pr.skip(skip);
    }
}

pub fn var(pr: &mut Program, st: &mut Stack, mem: &mut Memo) {
    let op = pr.current().unwrap();

    let var_name = op.word.clone().unwrap();

    let operand = st.pop();

    assert!(
        !mem.contains_key(&var_name),
        "variable {var_name} already defined"
    );

    mem.insert(var_name, operand);
}

pub fn get_op_executable() -> HashMap<String, Executable> {
    let mut executables = HashMap::<String, Executable>::new();

    executables.insert("JMP".to_string(), jmp);

    executables.insert("EXEC".to_string(), exec);

    executables.insert("MARK".to_string(), mark);

    executables.insert("PUSH".to_string(), push);

    executables.insert("SKIP".to_string(), skip);

    executables.insert("CSKIP".to_string(), cskip);

    executables.insert("VAR".to_string(), var);

    executables
}
