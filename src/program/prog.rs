use std::collections::BTreeMap;
use crate::program::Value;

type OperationName = &'static str;

const PUSH: OperationName = "PUSH";
const EXEC: OperationName = "EXEC";
const MARK: OperationName = "MARK";
const JMP: OperationName = "JMP";
const VAR: OperationName = "VAR";
const CSKIP: OperationName = "CSKIP";
const SKIP: OperationName = "SKIP";

pub struct Operation {
    pub name: OperationName,
    pub count: Option<usize>,
    pub word: Option<String>,
    pub value: Option<Value>,
}

impl Operation {
    pub fn new_value(name: OperationName, value: Value) -> Self {
        Self {
            name,
            value: Some(value),
            word: None,
            count: None,
        }
    }
    pub fn new_word(name: OperationName, word: String) -> Self {
        Self {
            name,
            word: Some(word),
            count: None,
            value: None,
        }
    }
    pub fn new_count(name: OperationName, count: usize) -> Self {
        Self {
            name,
            count: Some(count),
            value: None,
            word: None,
        }
    }
    pub fn new_word_count(name: OperationName, word: String, count: usize) -> Self {
        Self {
            name,
            word: Some(word),
            count: Some(count),
            value: None,
        }
    }
    pub fn to_string(&self) -> String {
        let mut sb = self.name.to_string();

        if self.word.is_some() {
            sb.push_str(format!(" {}", self.word.clone().unwrap()).as_str());
        }
        if self.value.is_some() {
            sb.push_str(format!(" {}", self.value.clone().unwrap().repr()).as_str());
        }
        if self.count.is_some() {
            sb.push_str(format!(" {}", self.count.unwrap().clone()).as_str());
        }

        sb
    }
}

pub struct Program {
    ops: Vec<Operation>,
    marks: BTreeMap<String, usize>,
    trace: Vec<usize>,
    op_idx: usize
}

impl Program {
    pub fn new() -> Self {
        Program {
            ops: vec![],
            trace: Vec::with_capacity(255),
            marks: BTreeMap::new(),
            op_idx: 0
        }
    }
    pub fn merge(&mut self, prog: Program) {
        self.ops.extend(prog.ops);
    }
    pub fn new_mark(&mut self, name: String) {
        self.ops.push(Operation::new_word(MARK, name.clone()));

        self.marks.insert(name, self.ops.len() - 1);
    }
    pub fn new_push(&mut self, value: Value) {
        self.ops.push(Operation::new_value(PUSH, value));
    }
    pub fn new_var(&mut self, name: String) {
        self.ops.push(Operation::new_word(VAR, name));
    }
    pub fn new_jmp(&mut self, name: String) {
        self.ops.push(Operation::new_word(JMP, name));
    }
    pub fn new_cskip(&mut self, num: usize) {
        self.ops.push(Operation::new_count(CSKIP, num));
    }
    pub fn new_skip(&mut self, num: usize) {
        self.ops.push(Operation::new_count(SKIP, num));
    }
    pub fn new_exec(&mut self, name: String, argc: usize) {
        self.ops.push(Operation::new_word_count(EXEC, name, argc));
    }
    pub fn is_end(&self) -> bool {
        self.op_idx > self.ops.len() - 1
    }
    pub fn finish_block(&mut self) {
        self.op_idx = match self.trace.pop() {
            Some(idx) => idx,
            None => self.ops.len(),
        };
    }
    pub fn next(&mut self) {
        self.op_idx += 1;

        if !self.is_end() {
            return;
        }

        self.finish_block();
    }
    pub fn current(&self) -> Option<&Operation> {
        if self.is_end() {
            return None;
        }

        self.ops.get(self.op_idx)
    }
    pub fn trace_back(&mut self) {
        self.trace.push(self.op_idx + 1);
    }
    pub fn skip(&mut self, num: usize) {
        if num == 0 && self.op_idx > 0 {
            self.op_idx -= 1;
            return;
        }
        self.op_idx += num;
    }
    pub fn jump_to_mark(&mut self, name: String) {
        let name_clone = name.clone();

        if let Some(op_id) = self.marks.get(&name_clone) {
            self.op_idx = *op_id;

            return;
        }

        panic!("segmentation fault, {name_clone} mark name not found")
    }
    pub fn jump_to_program_begin(&mut self) {
        self.jump_to_mark("#MAIN".to_string());
    }
    pub fn to_string(&self) -> String {
        let mut string = String::new();

        let mut i = 0;

        for op in &self.ops {
            string.push_str(format!("{}: {}\n", i, op.to_string()).as_str());
            i += 1;
        }

        string
    }
}
