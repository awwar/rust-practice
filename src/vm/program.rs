use crate::procedure::Procedure;
use crate::vm::operation::{Operation, OperationName};
use crate::vm::Value;
use std::collections::BTreeMap;

pub struct Program {
    ops: Vec<Operation>,
    procedures: Vec<Box<dyn Procedure>>,
    marks: Vec<usize>,
    marks_map: BTreeMap<String, usize>,
    variables_map: BTreeMap<String, usize>,
    trace: Vec<usize>,
    op_idx: usize,
}

impl Program {
    pub fn new() -> Self {
        Program {
            ops: vec![],
            procedures: vec![],
            marks: vec![0, 1],
            trace: Vec::with_capacity(255),
            marks_map: BTreeMap::new(),
            variables_map: BTreeMap::new(),
            op_idx: 0,
        }
    }
    pub fn get_memo_size(&self) -> usize {
        self.variables_map.len()
    }
    pub fn get_procedure(&self, id: usize) -> &Box<dyn Procedure> {
        &self.procedures[id]
    }
    pub fn new_mark(&mut self, name: String) {
        let len = self.marks_map.len();
        let v = self.marks_map.entry(name).or_insert(len);

        self.ops.push(Operation::new_id(OperationName::MARK, *v));

        if self.marks.len() < *v {
            self.marks.resize(*v + 1, 0);
        }
        self.marks[*v] = self.ops.len() - 1;
    }
    pub fn new_push(&mut self, value: Value) {
        self.ops
            .push(Operation::new_value(OperationName::PUSH, value));
    }
    pub fn new_push_var(&mut self, name: String) {
        let len = self.variables_map.len();
        let v = self.variables_map.entry(name).or_insert(len);

        self.ops.push(Operation::new_value(
            OperationName::PUSH,
            Value::Variable(*v),
        ));
    }
    pub fn new_heap(&mut self, name: String) {
        let len = self.variables_map.len();
        let v = self.variables_map.entry(name).or_insert(len);

        self.ops.push(Operation::new_value(
            OperationName::HEAP,
            Value::Variable(*v),
        ));
    }
    pub fn new_jmp(&mut self, name: String) {
        let len = self.marks_map.len();
        let v = self.marks_map.entry(name).or_insert(len);

        self.ops.push(Operation::new_id(OperationName::JMP, *v));
    }
    pub fn new_cskip(&mut self, num: usize) {
        self.ops
            .push(Operation::new_count(OperationName::CSKIP, num));
    }
    pub fn new_skip(&mut self, num: usize) {
        self.ops
            .push(Operation::new_count(OperationName::SKIP, num));
    }
    pub fn new_exec(&mut self, proc: Box<dyn Procedure>, argc: usize) {
        let mut operation_id: Option<usize> = None;

        for (i, procedure) in self.procedures.iter().enumerate() {
            if proc.spec().debug().eq(&procedure.spec().debug()) {
                operation_id = Some(i);

                break;
            }
        }

        if operation_id.is_none() {
            self.procedures.push(proc);

            operation_id = Some(self.procedures.len() - 1);
        }

        self.ops.push(Operation::new_word_count(
            OperationName::EXEC,
            operation_id.unwrap(),
            argc,
        ));
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
    pub fn next(&mut self) -> bool {
        self.op_idx += 1;

        if !self.is_end() {
            return true;
        }

        self.finish_block();

        !self.is_end()
    }
    pub fn current(&self) -> &Operation {
        unsafe { self.ops.get_unchecked(self.op_idx) }
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
    pub fn jump_to_mark(&mut self, id: usize) {
        self.op_idx = self.marks[id];
    }
    pub fn jump_to_program_begin(&mut self) {
        self.jump_to_mark(self.marks_map["#MAIN"]);
    }
    pub fn to_string(&self) -> String {
        let mut string = String::new();

        string.push_str("-- marks:\n");

        for (name, idx) in &self.marks_map {
            string.push_str(format!("{name}: {}\n", self.marks[*idx]).as_str());
        }

        string.push_str("-- variable:\n");

        for (name, idx) in &self.variables_map {
            string.push_str(format!("{name}: ${idx}\n").as_str());
        }

        let mut i = 0;

        string.push_str("-- procedures:\n");

        for proc in &self.procedures {
            string.push_str(format!("{}: {}\n", i, proc.spec().debug()).as_str());
            i += 1;
        }

        i = 0;

        string.push_str("-- ops:\n");

        for op in &self.ops {
            string.push_str(format!("{}: {}\n", i, op.to_string()).as_str());
            i += 1;
        }

        string
    }
}
