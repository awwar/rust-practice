use crate::program::{Program, Value};
use crate::vm::vm::{Memo, Stack};

pub type Executable = fn(&mut Program, &mut Stack, &mut Memo);

pub fn jmp(pr: &mut Program, _: &mut Stack, _: &mut Memo) {
    let mark_name = pr.current().unwrap().word.clone().unwrap();
    pr.trace_back();
    pr.jump_to_mark(mark_name.as_str());
}

pub fn exec(pr: &mut Program, st: &mut Stack, _: &mut Memo) {
    let op = pr.current().unwrap();

    let argc = op.count.unwrap();
    let proc = pr.get_procedure(op.id.unwrap());

    proc.execute(argc, st).unwrap();
}

pub fn mark(pr: &mut Program, _: &mut Stack, _: &mut Memo) {
    pr.finish_block();
    pr.skip(0);
}

pub fn push(pr: &mut Program, st: &mut Stack, mem: &mut Memo) {
    let op = pr.current().unwrap();

    if let Some(val) = &op.value {
        match val {
            Value::Variable(var) => st.push(mem[*var].clone()),
            value => st.push(value.clone())
        }

        return;
    }

    panic!("invalid op push - expecting value");
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

    let Some(Value::Variable(var)) = op.value else {panic!("invalid op var - expecting value")};
    mem[var] = st.pop();
}

pub fn get_op_executable(name: &str) -> Executable {
    match name {
        "JMP" => jmp,
        "EXEC" => exec,
        "MARK" => mark,
        "PUSH" => push,
        "SKIP" => skip,
        "CSKIP" => cskip,
        "VAR" => var,
        _ => panic!("Unknown variable name"),
    }
}
