use crate::control_structures::get_control_structures;
use crate::lexer::{TokenName, TokenStream};
use crate::parser::node::Node;

pub struct Parser {
    last_position: usize,
    current_position: usize,
    stream: TokenStream,
}

impl Parser {
    pub fn new(stream: TokenStream, current_position: usize, last_position: usize) -> Self {
        Parser {
            last_position,
            current_position,
            stream,
        }
    }

    pub fn new_from_stream(stream: TokenStream) -> Self {
        Self::new(stream, 0, usize::MAX)
    }

    pub fn parse_program(&mut self) -> Result<Node, String> {
        let mut list = Vec::<Node>::new();

        loop {
            if !self.stream.has(self.current_position) {
                break;
            }

            let node = self.subparse_flow_declaration()?;

            list.push(node);

            self.current_position += 1;
        }

        Ok(Node::new_program(list))
    }

    pub fn subparse_flow_declaration(&mut self) -> Result<Node, String> {
        let token = self.stream.get(self.current_position)?;

        if token.name != TokenName::Word || !token.starts_with("#") {
            return Err(self.error(
                self.current_position,
                "flow declaration must start with # and has argument and return value",
            ));
        }

        let mut list = Vec::<Node>::new();

        let next_token = self.stream.get(self.current_position + 1)?;

        if next_token.name != TokenName::Bracket {
            return Err(self.error(next_token.at, "word token uses only in function context"));
        }

        let args = self.subparse_list_in_bracers(None)?;

        let return_param = self.subparse_word()?;

        list.extend(args);
        list.push(return_param);

        loop {
            let next_token = match self.stream.get(self.current_position + 1) {
                Ok(token) => token,
                Err(_) => break,
            };

            if next_token.starts_with("#") {
                break;
            }

            let node = self.subparse_node()?;

            list.push(node);
        }

        Ok(Node::new_flow_declaration(token.value, list, token.at))
    }

    pub fn subparse_variable_name(&mut self) -> Result<Node, String> {
        self.current_position += 1;
        let token = self.stream.get(self.current_position)?;

        if token.name != TokenName::Word || !token.starts_with("$") {
            return Err(self.error(self.current_position, "variable must start with $"));
        }

        Ok(Node::new_variable(token.value, token.at))
    }

    pub fn subparse_one_in_bracers(&mut self) -> Result<Node, String> {
        let sub_nodes = self.subparse_list_in_bracers(Some(1))?;

        if sub_nodes.len() != 1 {
            return Err(self.error(self.current_position, "expected 1 sub expression"));
        }

        Ok(sub_nodes.first().unwrap().clone())
    }

    pub fn subparse_node(&mut self) -> Result<Node, String> {
        self.current_position += 1;
        let token = self.stream.get(self.current_position)?;

        if token.name != TokenName::Word {
            return Err(self.error(token.at, "node declaration must start with node name"));
        }

        let control_structure_name = token.value.to_uppercase();
        let proc = get_control_structures(&control_structure_name);

        proc.parse(token.clone(), self)
    }

    pub fn subparse_list_in_bracers(&mut self, length: Option<usize>) -> Result<Vec<Node>, String> {
        let start_token = self.stream.get(self.current_position)?;

        self.current_position += 1;

        let open_bracer = self.stream.get(self.current_position)?;

        if open_bracer.name != TokenName::Bracket {
            return Err(self.error(start_token.at, "word token uses only in function context"));
        }

        let end_bracer_position = self
            .stream
            .search_idx_of_closed_bracer(self.current_position)?;

        let mut sub_nodes: Vec<Node> = Vec::new();

        if self.current_position != end_bracer_position - 1 {
            let mut sub_parser = Parser::new(
                self.stream.clone(),
                self.current_position + 1,
                end_bracer_position - 1,
            );

            sub_nodes = sub_parser.subparse_expressions()?;
        }

        if length.is_some() && sub_nodes.len() != length.unwrap() {
            return Err(self.error(
                start_token.at,
                format!(
                    "expected {} nodes, got {}",
                    sub_nodes.len(),
                    length.unwrap()
                )
                .as_str(),
            ));
        }

        self.current_position = end_bracer_position;

        Ok(sub_nodes)
    }

    pub fn subparse_expressions(&mut self) -> Result<Vec<Node>, String> {
        let mut list = Vec::<Node>::new();

        loop {
            let token = self.stream.get(self.current_position)?;

            match token.name {
                TokenName::Comma => {
                    let mut sub_parser = Self::new(
                        self.stream.clone(),
                        self.current_position + 1,
                        self.last_position,
                    );
                    let sub_nodes = sub_parser.subparse_expressions()?;
                    list.extend(sub_nodes);
                    break;
                }
                TokenName::Word => {
                    if token.starts_with("#") {
                        list.push(Node::new_flow_link(token.value, token.at));
                    } else if token.starts_with("$") {
                        list.push(Node::new_variable(token.value, token.at));
                    } else {
                        let sub_nodes = self.subparse_list_in_bracers(None)?;

                        list.push(Node::new_operation(token.value, sub_nodes, token.at));
                    }
                }
                TokenName::Bracket => {
                    self.current_position -= 1;
                    let sub_node = self.subparse_one_in_bracers()?;
                    list.push(sub_node.clone_with_priority(0));
                }
                TokenName::Operator => {
                    list.push(Node::new_operation(token.value, vec![], token.at));
                }
                TokenName::Number => {
                    list.push(Node::new_number(token.value, token.at));
                }
                TokenName::String => {
                    list.push(Node::new_string(token.value, token.at));
                }
                _ => {
                    return Err(self.error(token.at, "unexpected token"));
                }
            }

            if self.current_position == self.last_position {
                break;
            }

            self.current_position += 1;
        }

        Ok(self.prioritize(list))
    }

    pub fn subparse_word(&mut self) -> Result<Node, String> {
        self.current_position += 1;
        let next_token = self.stream.get(self.current_position)?;

        if next_token.name != TokenName::Word {
            return Err(self.error(
                self.current_position,
                "word token uses only in function context",
            ));
        }

        Ok(Node::new_constant(next_token.value, self.current_position))
    }

    fn error(&self, _: usize, message: &str) -> String {
        message.to_string()
    }

    fn prioritize(&self, input_list: Vec<Node>) -> Vec<Node> {
        let mut target_priority = 5; // 4 + 1
        let mut pointer: usize = 0;

        let mut list = input_list.clone();

        loop {
            let current_node = &mut if let Some(node) = list.get(pointer) {
                node.clone()
            } else {
                pointer = 0;
                if target_priority == 0 {
                    break;
                }
                target_priority -= 1;

                continue;
            };
            pointer += 1;

            if current_node.get_priority() != target_priority {
                continue;
            }

            current_node.deprioritize();

            // if list.get(pointer - 1).is_none() {
            //     continue;
            // }

            for transformer in &[math_operations, function_call] {
                if let Some(lst) = transformer(list.clone(), pointer) {
                    list = lst;

                    break;
                }
            }
        }

        list
    }
}

fn math_operations(mut list: Vec<Node>, pointer: usize) -> Option<Vec<Node>> {
    // 1 + 1
    if pointer < 1 || list.len() < 2 + pointer {
        return None;
    }

    let lft = list.get(pointer - 1).unwrap();
    let cur = list.get(pointer).unwrap();
    let rgt = list.get(pointer + 1).unwrap();

    if !cur.is_mathematical_operation() {
        return None;
    }

    let to = [Node::new_operation(
        cur.value.clone(),
        vec![lft.clone(), rgt.clone()],
        cur.token_position,
    )];

    list.splice(pointer - 1..pointer + 2, to);

    Some(list)
}

fn function_call(mut list: Vec<Node>, pointer: usize) -> Option<Vec<Node>> {
    // obj.method
    if pointer < 1 || list.len() < 2 + pointer {
        return None;
    }

    let lft = list.get(pointer - 1).unwrap();
    let cur = list.get(pointer).unwrap();
    let rgt = list.get(pointer + 1).unwrap();

    if !cur.is_call_operation() {
        return None;
    }

    let to = [Node::new_operation(
        rgt.value.clone(),
        vec![lft.clone()],
        cur.token_position,
    )];

    list.splice(pointer - 1..pointer + 2, to);

    Some(list)
}
