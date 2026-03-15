use crate::vm::program::Program;
use crate::vm::value::Value;
use crate::vm::vm::{Memo, Stack};
use std::fmt::Display;

pub type Executable = fn(&mut Program, &mut Stack, &mut Memo);

pub enum OperationName {
    PUSH,
    EXEC,
    MARK,
    JMP,
    HEAP,
    CSKIP,
    SKIP,
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

impl Display for OperationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            OperationName::PUSH => "PUSH".to_string(),
            OperationName::EXEC => "EXEC".to_string(),
            OperationName::MARK => "MARK".to_string(),
            OperationName::JMP => "JMP".to_string(),
            OperationName::HEAP => "HEAP".to_string(),
            OperationName::CSKIP => "CSKIP".to_string(),
            OperationName::SKIP => "SKIP".to_string(),
        };
        write!(f, "{str}")
    }
}

pub struct Operation {
    pub name: OperationName,
    pub count: usize,
    pub id: usize,
    pub value: Value,
}

impl Operation {
    pub fn new_value(name: OperationName, value: Value) -> Self {
        Self {
            name,
            value,
            id: 0,
            count: 0,
        }
    }
    pub fn new_count(name: OperationName, count: usize) -> Self {
        Self {
            name,
            count,
            id: 0,
            value: Value::Null,
        }
    }
    pub fn new_word_count(name: OperationName, id: usize, count: usize) -> Self {
        Self {
            name,
            count,
            id,
            value: Value::Null,
        }
    }
    pub fn new_id(name: OperationName, id: usize) -> Self {
        Self {
            name,
            count: 0,
            id,
            value: Value::Null,
        }
    }
    pub fn to_string(&self) -> String {
        let mut sb = self.name.to_string();

        sb.push_str(format!(" id: {},", self.id.clone()).as_str());
        sb.push_str(format!(" count: {},", self.count.clone()).as_str());
        sb.push_str(format!(" value: {}", self.value.repr()).as_str());

        sb
    }
}

fn jmp(pr: &mut Program, _: &mut Stack, _: &mut Memo) {
    pr.trace_back();
    pr.jump_to_mark(pr.current().id);
}

fn exec(pr: &mut Program, st: &mut Stack, _: &mut Memo) {
    let op = pr.current();
    let proc = pr.get_procedure(op.id);

    proc.execute(op.count, st);
}

fn mark(pr: &mut Program, _: &mut Stack, _: &mut Memo) {
    pr.finish_block();
    pr.skip(0);
}

fn push(pr: &mut Program, st: &mut Stack, mem: &mut Memo) {
    match &pr.current().value {
        Value::Variable(var) => st.push(mem[*var].clone()),
        value => st.push(value.clone()),
    }
}

fn skip(pr: &mut Program, _: &mut Stack, _: &mut Memo) {
    pr.skip(pr.current().count);
}

fn cskip(pr: &mut Program, st: &mut Stack, _: &mut Memo) {
    let operand = st.pop();

    let Value::Bool(cond) = operand else {
        panic!("cskip condition must be a bool");
    };

    if cond {
        let skip = pr.current().count;

        pr.skip(skip);
    }
}

fn heap(pr: &mut Program, st: &mut Stack, mem: &mut Memo) {
    let Value::Variable(var) = pr.current().value else {
        panic!("invalid pr.current() var - expecting value")
    };
    mem[var] = st.pop();
}
