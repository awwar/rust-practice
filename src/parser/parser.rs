use crate::lexer::{TokenStream, Token, TokenName};
use crate::parser::node::{Node};

pub struct Parser {
    first_position: usize,
    last_position: usize,
    current_position: usize,
    stream: TokenStream,
}

pub type ProcedureParser = dyn Fn(&Token, &Parser) -> Result<Node, String>;

impl Parser {
    pub fn new(stream: TokenStream, first_position: usize, last_position: usize) -> Self {
        Parser {
            stream,
            first_position,
            last_position,
            current_position: first_position,
        }
    }

    pub fn new_from_stream(stream: TokenStream) -> Self {
        Self::new(stream, 0, usize::MAX)
    }

    pub fn parse_program(&mut self) -> Result<Node, String> {
        let mut list = Vec::<Node>::new();

        loop {
            match self.stream.get(self.current_position) {
                None => break,
                Some(token) => token
            };

            match self.subparse_flow_declaration() {
                Ok(node) => list.push(node),
                Err(err) => return Err(err),
            }

            self.current_position += 1;
        }

        Ok(Node::new_program(list))
    }

    pub fn subparse_flow_declaration(&mut self) -> Result<Node, String> {
        let token = match self.stream.get(self.current_position) {
            None => return Err(format!("unable to find token at {:?}", self.current_position)),
            Some(token) => token
        };

        if token.name != TokenName::Word || !token.starts_with("#") {
            return Err(self.error(self.current_position, "flow declaration must start with # and has argument and return value"))
        }

        let mut list = Vec::<Node>::new();

        let next_token  = match self.stream.get(self.current_position+1) {
            None => return Err(format!("unable to find token at {:?}", self.current_position+1)),
            Some(token) => token
        };

        if next_token.name != TokenName::Bracket {
            return Err(self.error(next_token.at, "word token uses only in function context"))
        }

        let args_candidates = self.subparse_list_in_bracers(None);
        if args_candidates.is_err() {
            return Err(args_candidates.err().unwrap())
        }

        list.extend(args_candidates.unwrap());

        let return_param = match self.subparse_word() {
            Err(err) => return Err(err),
            Ok(node) => node
        };

        list.push(return_param);

        Ok(Node::new_flow_declaration(token.value, list, token.at))
    }

    pub fn subparse_one_in_bracers(&mut self) -> Result<Node, String> {
        let sub_node_candidates = self.subparse_list_in_bracers(Some(1));

        if sub_node_candidates.is_err() {
            return Err(sub_node_candidates.err().unwrap())
        }

        let sub_nodes = sub_node_candidates.unwrap();

        if sub_nodes.len() != 1 {
            return Err(self.error(self.current_position, "expected 1 sub expression"))
        }

        Ok(sub_nodes.first().unwrap().clone())
    }

    pub fn subparse_list_in_bracers(&mut self, length: Option<usize>) -> Result<Vec<Node>, String> {
        let start_token = self.stream.get(self.current_position).unwrap();

        self.current_position += 1;

        let open_bracer = match self.stream.get(self.current_position) {
            None => return Err(self.error(start_token.at, "expected next token")),
            Some(token) => token
        };

        if open_bracer.name != TokenName::Bracket {
            return Err(self.error(start_token.at, "word token uses only in function context"))
        }

        let end_bracer_position = match self.stream.search_idx_of_closed_bracer(self.current_position) {
            None => return Err(self.error(open_bracer.at, "missing closed bracer")),
            Some(end_bracer) => end_bracer
        };

        let mut sub_nodes: Vec<Node> = Vec::new();

        if self.current_position != end_bracer_position - 1 {
            let mut sub_parser = Parser::new(self.stream.clone(), self.current_position+1, end_bracer_position-1);

            let sub_nodes_candidate = sub_parser.subparse_expressions();
            if sub_nodes_candidate.is_err() {
                return sub_nodes_candidate
            }

            sub_nodes = sub_nodes_candidate.unwrap()
        }

        if length.is_some() && sub_nodes.len() != length.unwrap() {
            return Err(self.error(start_token.at, format!("expected {} nodes, got {}", sub_nodes.len(), length.unwrap()).as_str()))
        }

        self.current_position = end_bracer_position;

        Ok(sub_nodes)
    }

    pub fn subparse_expressions(&mut self) -> Result<Vec<Node>, String> {
        let mut list = Vec::<Node>::new();

        loop {
            let token = self.stream.get(self.current_position);
            if token.is_none() {
                let last_token = self.stream.get(self.current_position - 1).unwrap();
                return Err(self.error(last_token.at, "cant find token"));
            }
            let token = token.unwrap();

            match token.name {
                TokenName::Comma => {
                    let mut sub_parser = Self::new(self.stream.clone(), self.current_position + 1, self.last_position);
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
                        match self.subparse_list_in_bracers(None) {
                            Err(e) => return Err(e),
                            Ok(sub_nodes) => {
                                list.push(Node::new_operation(token.value, sub_nodes, token.at));
                            }
                        }
                    }
                }
                TokenName::Bracket => {

                }
                TokenName::Operator => {

                }
                TokenName::Number => {

                }
                TokenName::String => {

                }
                _ => {
                    // Обработка других типов токенов аналогично
                }
            }

            if self.current_position == self.last_position {
                break;
            }

            self.current_position += 1;
        }

        // let mut target_priority = 5; // 4 + 1

        // loop {
        //     list.next();
        //
        //     if list.is_end() {
        //         list.rewind();
        //         if target_priority == 0 {
        //             break;
        //         }
        //         target_priority -= 1;
        //     }
        //
        //     let current_node = list.current();
        //
        //     if current_node.get_priority() != target_priority {
        //         continue;
        //     }
        //
        //     current_node.deprioritize();
        //
        //     if list.left().is_none() {
        //         continue;
        //     }
        //
        //     for transformer in &transformers {
        //         let (is_replaced, err) = transformer(&list);
        //         if err.is_err() {
        //             return Err(self.error(current_node.token_position, err.unwrap_err()));
        //         }
        //         if is_replaced {
        //             break;
        //         }
        //     }
        // }

        Ok(list)
    }

    pub fn subparse_word(&mut self) -> Result<Node, String> {
        self.current_position += 1;
        let next_token  = match self.stream.get(self.current_position) {
            None => return Err(format!("unable to find token at {:?}", self.current_position)),
            Some(token) => token
        };

        if next_token.name != TokenName::Word {
            return Err(self.error(self.current_position, "word token uses only in function context"))
        }

        Ok(Node::new_constant(next_token.value, self.current_position))
    }

    fn error(&self, position: usize, message: &str) -> String {
        crate::util::new_error(position, "".to_string(), message)
    }
}