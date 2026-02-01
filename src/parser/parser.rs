use crate::lexer::{TokenStream, Token, TokenName};
use crate::parser::node::Node;

pub struct Parser {
    first_position: usize,
    last_position: usize,
    current_position: usize,
    stream: TokenStream,
}

// pub type ProcedureParser = dyn Fn(&Token, &Parser) -> Result<Node, String>;
//
// impl Parser {
//     pub fn new(stream: TokenStream, first_position: usize, last_position: usize) -> Self {
//         Parser {
//             stream,
//             first_position,
//             last_position,
//             current_position: first_position,
//         }
//     }
//
//     pub fn new_from_stream(stream: TokenStream) -> Self {
//         Self::new(stream, 0, stream.length() - 1)
//     }
//
//     pub fn parse_program(&mut self) -> Result<Node, String> {
//         let mut list = NodeList::new();
//
//         loop {
//             match self.subparse_flow_declaration() {
//                 Ok(node) => list.push(node),
//                 Err(err) => return Err(err),
//             }
//
//             if self.current_position == self.last_position {
//                 break;
//             }
//
//             self.current_position += 1;
//         }
//
//         Ok(create_as_program(list.result()))
//     }
//
//     pub fn subparse_expressions(&mut self) -> Result<Vec<Node>, String> {
//         let mut list = NodeList::new();
//
//         loop {
//             let token = self.stream.get(self.current_position);
//             if token.is_none() {
//                 let last_token = self.stream.get(self.current_position - 1).unwrap();
//                 return Err(self.error(last_token.position, "cant find token"));
//             }
//             let token = token.unwrap();
//
//             match token.name {
//                 TokenName::Comma => {
//                     let mut sub_parser = Self::new(self.stream.clone(), self.current_position + 1, self.last_position);
//                     let sub_nodes = sub_parser.subparse_expressions()?;
//                     list.push(sub_nodes);
//                     break;
//                 }
//                 _ => {
//                     // Обработка других типов токенов аналогично
//                 }
//             }
//
//             if self.current_position == self.last_position {
//                 break;
//             }
//
//             self.current_position += 1;
//         }
//
//         let mut target_priority = 5; // 4 + 1
//
//         loop {
//             list.next();
//
//             if list.is_end() {
//                 list.rewind();
//                 if target_priority == 0 {
//                     break;
//                 }
//                 target_priority -= 1;
//             }
//
//             let current_node = list.current();
//
//             if current_node.get_priority() != target_priority {
//                 continue;
//             }
//
//             current_node.deprioritize();
//
//             if list.left().is_none() {
//                 continue;
//             }
//
//             for transformer in &transformers {
//                 let (is_replaced, err) = transformer(&list);
//                 if err.is_err() {
//                     return Err(self.error(current_node.token_position, err.unwrap_err()));
//                 }
//                 if is_replaced {
//                     break;
//                 }
//             }
//         }
//
//         Ok(list.result())
//     }
//
//     pub fn subparse_one_in_bracers(&mut self) -> Result<Node, String> {
//         let sub_nodes = self.subparse_list_in_bracers(1)?;
//         Ok(sub_nodes[0].clone()) // Предполагая, что Node реализует Clone
//     }
//
//     pub fn subparse_variable_name(&mut self) -> Result<Node, String> {
//         self.current_position += 1;
//         let token = self.stream.get(self.current_position);
//         // Обработка токена
//         Ok(Node {}) // Замените на реальную логику
//     }
//
//     fn error(&self, position: usize, message: &str) -> String {
//         format!("Error at position {}: {}", position, message)
//     }
// }