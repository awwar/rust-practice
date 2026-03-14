use crate::program::{OperationName, Program, Value};
use crate::vm::vm::{Memo, Stack};

pub type Executable = fn(&mut Program, &mut Stack, &mut Memo);

pub fn jmp(pr: &mut Program, _: &mut Stack, _: &mut Memo) {
    pr.trace_back();
    pr.jump_to_mark(pr.current().id);
}

pub fn exec(pr: &mut Program, st: &mut Stack, _: &mut Memo) {
    let op = pr.current();
    let proc = pr.get_procedure(op.id);

    proc.execute(op.count, st);
}

pub fn mark(pr: &mut Program, _: &mut Stack, _: &mut Memo) {
    pr.finish_block();
    pr.skip(0);
}

pub fn push(pr: &mut Program, st: &mut Stack, mem: &mut Memo) {
    match &pr.current().value {
        Value::Variable(var) => st.push(mem[*var].clone()),
        value => st.push(value.clone()),
    }
}

pub fn skip(pr: &mut Program, _: &mut Stack, _: &mut Memo) {
    pr.skip(pr.current().count);
}

pub fn cskip(pr: &mut Program, st: &mut Stack, _: &mut Memo) {
    let operand = st.pop();

    let Value::Bool(cond) = operand else {
        panic!("cskip condition must be a bool");
    };

    if cond == true {
        let skip = pr.current().count;

        pr.skip(skip);
    }
}

pub fn heap(pr: &mut Program, st: &mut Stack, mem: &mut Memo) {
    let Value::Variable(var) = pr.current().value else {
        panic!("invalid pr.current() var - expecting value")
    };
    mem[var] = st.pop();
}

pub fn get_op_executable(name: &OperationName) -> Executable {
    match name {
        OperationName::PUSH => push,
        OperationName::EXEC => exec,
        OperationName::MARK => mark,
        OperationName::JMP => jmp,
        OperationName::HEAP => heap,
        OperationName::CSKIP => cskip,
        OperationName::SKIP => skip,
    }
}
