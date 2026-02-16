use std::collections::LinkedList;

type OperationName = &'static str;

const PUSH: OperationName = "PUSH";
const EXEC: OperationName = "EXEC";
const MARK: OperationName = "MARK";
const JMP: OperationName = "JMP";
const VAR: OperationName = "VAR";
const CSKIP: OperationName = "CSKIP";
const SKIP: OperationName = "SKIP";

struct Operation {
    name: OperationName,
    count: usize,
    word: String,
}

impl Operation {
    pub fn new_word(name: OperationName, word: String) -> Self {
        Self {
            name,
            word,
            count: 0,
        }
    }
    pub fn new_count(name: OperationName, count: usize) -> Self {
        Self {
            name,
            word: "".to_string(),
            count,
        }
    }
    pub fn new_word_count(name: OperationName, word: String, count: usize) -> Self {
        Self { name, word, count }
    }
    fn to_string(&self) -> String {
        format!("{} {} {}", self.name, self.word, self.count)
    }
}

pub struct Program {
    ops: Vec<Operation>,
    trace: LinkedList<usize>,
    op_idx: usize,
}

impl Program {
    pub fn new() -> Self {
        Program {
            ops: vec![],
            trace: LinkedList::new(),
            op_idx: 0,
        }
    }
    pub fn merge(&mut self, prog: Program) {
        self.ops.extend(prog.ops);
    }
    pub fn new_mark(&mut self, name: String) {
        self.ops.push(Operation::new_word(MARK, name));
    }
    pub fn new_push(&mut self, name: String) {
        self.ops.push(Operation::new_word(PUSH, name));
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
        self.op_idx = match self.trace.pop_front() {
            Some(idx) => idx,
            None => self.ops.len(),
        };
    }
    pub fn next(&mut self) {
        self.op_idx += 1;

        if self.is_end() {
            return;
        }

        self.finish_block();
    }
    pub fn current(&mut self) -> Option<&Operation> {
        if self.is_end() {
            return None;
        }

        self.ops.get(self.op_idx)
    }
    pub fn trace_back(&mut self) {
        self.trace.push_front(self.op_idx + 1)
    }
    pub fn skip(&mut self, num: usize) {
        self.op_idx += num
    }
    pub fn jump_to_mark(&mut self, name: String) {
        for op in self.ops.iter() {
            if op.name != MARK {
                continue;
            }

            if op.word == name {
                self.op_idx = op.count;
            }
        }

        panic!("segmentation fault, {} mark name not found", name)
    }
    pub fn jump_to_program_begin(&mut self) {
        self.jump_to_mark("#MAIN".to_string());
    }
    pub fn to_string(&self) -> String {
        let mut string = String::new();

        let mut i = 0;

        for op in self.ops.iter() {
            string.push_str(format!("{}: {}\n", i, op.to_string()).as_str());
            i += 1;
        }

        string
    }
}
