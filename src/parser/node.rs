use crate::program::{Value, ValueConverter, ValueType};
use std::cmp::PartialEq;

#[derive(PartialEq, Clone)]
enum NodeType {
    Operation,
    Constant,
    Variable,
    FlowLink,
    FlowDeclaration,
    FlowBranchesDeclaration,
    Program,
}

#[derive(Clone)]
pub struct Node {
    node_type: NodeType,
    pub(crate) value: Box<dyn ValueConverter>,
    params: Vec<Node>,
    priority: usize,
    pub(crate) token_position: usize,
}

const OPERATION_PRIORITY: [&'static str; 9] = ["+", "-", "*", "/", ">", "<", "=", "^", "."];

impl Node {
    pub(crate) fn new_program(params: Vec<Self>) -> Self {
        Self {
            node_type: NodeType::Program,
            value: Box::new(Value::<String>("root".to_string())),
            params,
            priority: 4,
            token_position: 0,
        }
    }

    pub(crate) fn new_constant(value: String, token_position: usize) -> Self {
        Self {
            node_type: NodeType::Constant,
            value: Box::new(Value::<String>(value)),
            params: vec![],
            priority: 4,
            token_position,
        }
    }

    pub(crate) fn new_operation(operation: String, params: Vec<Self>, token_position: usize) -> Self {
        let mut node = Node {
            node_type: NodeType::Operation,
            value: Box::new(Value::<String>(operation.to_string())),
            params,
            priority: OPERATION_PRIORITY.iter().position(|n| n.eq(&operation)).unwrap_or(0) + 1,
            token_position,
        };

        if !node.is_mathematical_operation() {
            node.priority = 4
        }

        node
    }

    pub(crate) fn new_number(value: String, token_position: usize) -> Self {
        let value_obj: Box<dyn ValueConverter>;

        if value.contains(".") {
            value_obj = Box::new(Value::<f64>(value.parse::<f64>().unwrap()));
        } else {
            value_obj = Box::new(Value::<i64>(value.parse::<i64>().unwrap()));
        }

        Self {
            node_type: NodeType::Constant,
            value: value_obj,
            params: vec![],
            priority: 4,
            token_position,
        }
    }

    pub(crate) fn new_string(value: String, token_position: usize) -> Self {
        Self {
            node_type: NodeType::Constant,
            value: Box::new(Value::<String>(value)),
            params: vec![],
            priority: 4,
            token_position,
        }
    }

    pub(crate) fn new_flow_declaration(value: String, params: Vec<Self>, token_position: usize) -> Self {
        Self {
            node_type: NodeType::FlowDeclaration,
            value: Box::new(Value::<String>(value)),
            params,
            priority: 4,
            token_position,
        }
    }

    fn new_flow_branches_declaration(value: String, params: Vec<Self>, token_position: usize) -> Self {
        Self {
            node_type: NodeType::FlowBranchesDeclaration,
            value: Box::new(Value::<String>(value)),
            params,
            priority: 4,
            token_position,
        }
    }

    pub(crate) fn new_flow_link(value: String, token_position: usize) -> Self {
        Self {
            node_type: NodeType::FlowLink,
            value: Box::new(Value::<String>(value)),
            params: vec![],
            priority: 4,
            token_position,
        }
    }

    pub(crate) fn new_variable(value: String, token_position: usize) -> Self {
        Self {
            node_type: NodeType::Variable,
            value: Box::new(Value::<String>(value)),
            params: vec![],
            priority: 4,
            token_position,
        }
    }
    pub(crate) fn format(&self, indent: i32) -> String {
        let string_indent = "    ".repeat(indent as usize);

        let mut branches = "".to_string();

        for n in self.params.iter() {
            let substr: String = n.clone().format(indent + 1);
            branches += format!("{}└── {}", string_indent, substr).as_str();
        }

        format!("{}\n{}", self.value.raw(), branches)
    }

    pub fn clone_with_priority(self, priority: usize) -> Self {
        let mut self_clone = self.clone();
        self_clone.priority = priority;
        return self_clone;
    }

    pub(crate) fn get_priority(&self) -> usize {
        self.priority
    }

    fn set_sub_node(mut self, offset: usize, node: Self) {
        self.params.insert(offset, node)
    }

    fn set_only_child(mut self, node: Self) {
        self.params = vec![node];
    }

    pub(crate) fn deprioritize(&mut self) {
        self.priority = 0
    }

    pub(crate) fn is_mathematical_operation(&self) -> bool {
        if self.node_type != NodeType::Operation {
            return false;
        }

        let val = self.value.to_string();
        if val.is_err() {
            return false;
        }

        let raw = val.unwrap().raw();

        OPERATION_PRIORITY.contains(&&**&raw) && raw.ne(".")
    }

    pub(crate) fn is_not_call_operation(&self) -> bool {
        !self.is_call_operation()
    }

    pub(crate) fn is_call_operation(&self) -> bool {
        if self.node_type != NodeType::Operation {
            return false;
        }

        let val = self.value.to_string();
        if val.is_err() {
            return false;
        }

        let raw = val.unwrap().raw();

        raw.eq(".")
    }

    fn is_negatable(&self) -> bool {
        self.is_function() || self.is_number()
    }

    fn is_function(&self) -> bool {
        self.node_type == NodeType::Operation && !self.is_mathematical_operation() && !self.is_call_operation()
    }

    fn is_number(&self) -> bool {
        self.value.type_name() == ValueType::Float || self.value.type_name() == ValueType::Float
    }

    fn is_minus_or_plus(&self) -> bool {
        self.node_type == NodeType::Operation && (self.value.raw().contains(&['+', '-']))
    }

    pub(crate) fn is_flow_link(&self) -> bool {
        self.node_type == NodeType::FlowLink
    }
}