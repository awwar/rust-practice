use std::cmp::PartialEq;

#[derive(PartialEq, Clone)]
pub enum NodeType {
    Operation,
    Constant,
    String,
    Float,
    Integer,
    Variable,
    FlowLink,
    FlowDeclaration,
    Program,
}

#[derive(Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub value: String,
    pub params: Vec<Node>,
    priority: usize,
    pub token_position: usize,
}

const OPERATION_PRIORITY: [&'static str; 9] = ["+", "-", "*", "/", ">", "<", "=", "^", "."];

impl Node {
    pub fn new_program(params: Vec<Self>) -> Self {
        Self {
            node_type: NodeType::Program,
            value: "ROOT".to_string(),
            params,
            priority: 4,
            token_position: 0,
        }
    }

    pub fn new_constant(value: String, token_position: usize) -> Self {
        Self {
            node_type: NodeType::Constant,
            value,
            params: vec![],
            priority: 4,
            token_position,
        }
    }

    pub fn new_operation(operation: String, params: Vec<Self>, token_position: usize) -> Self {
        let priority = OPERATION_PRIORITY.iter().position(|n| n.eq(&operation)).unwrap_or(0) + 1;

        let mut node = Node {
            node_type: NodeType::Operation,
            value: operation.to_uppercase(),
            params,
            priority,
            token_position,
        };

        if !node.is_mathematical_operation() {
            node.priority = 4
        }

        node
    }

    pub fn new_number(value: String, token_position: usize) -> Self {
        let parsed_value: String;

        if value.contains(".") {
            parsed_value = value.parse::<f64>().unwrap().to_string();
        } else {
            parsed_value = value.parse::<i64>().unwrap().to_string();
        }

        Self {
            node_type: if value.contains(".") { NodeType::Float } else { NodeType::Integer },
            value: parsed_value,
            params: vec![],
            priority: 4,
            token_position,
        }
    }

    pub fn new_string(value: String, token_position: usize) -> Self {
        Self {
            node_type: NodeType::String,
            value: value.strip_prefix('"').unwrap().strip_suffix('"').unwrap().to_string(),
            params: vec![],
            priority: 4,
            token_position,
        }
    }

    pub fn new_flow_declaration(value: String, params: Vec<Self>, token_position: usize) -> Self {
        Self {
            node_type: NodeType::FlowDeclaration,
            value: value.to_uppercase(),
            params,
            priority: 4,
            token_position,
        }
    }

    pub fn new_flow_link(value: String, token_position: usize) -> Self {
        Self {
            node_type: NodeType::FlowLink,
            value: value.to_uppercase(),
            params: vec![],
            priority: 4,
            token_position,
        }
    }

    pub fn new_variable(value: String, token_position: usize) -> Self {
        Self {
            node_type: NodeType::Variable,
            value: value.to_uppercase(),
            params: vec![],
            priority: 4,
            token_position,
        }
    }
    pub fn format(&self, indent: i32) -> String {
        let string_indent = " ".repeat((indent * 4) as usize);

        let mut branches = "".to_string();

        for n in self.params.iter() {
            let substr: String = n.clone().format(indent + 1);
            branches += format!("{}└── {}", string_indent, substr).as_str();
        }

        format!("{}\n{}", self.value, branches)
    }

    pub fn clone_with_priority(self, priority: usize) -> Self {
        let mut self_clone = self.clone();
        self_clone.priority = priority;
        return self_clone;
    }

    pub fn get_priority(&self) -> usize {
        self.priority
    }

    pub fn deprioritize(&mut self) {
        self.priority = 0
    }

    pub fn is_mathematical_operation(&self) -> bool {
        if self.node_type != NodeType::Operation {
            return false;
        }

        OPERATION_PRIORITY.contains(&&*self.value) && self.value.ne(".")
    }

    pub fn is_call_operation(&self) -> bool {
        if self.node_type != NodeType::Operation {
            return false;
        }

        self.value.eq(".")
    }

    pub fn is_flow_link(&self) -> bool {
        self.node_type == NodeType::FlowLink
    }
}