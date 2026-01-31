use crate::lexer::{TokenStream, Token};

#[allow(dead_code)]
struct Parser {
    first_position: usize,
    last_position: usize,
    current_position: usize,

    stream: TokenStream,
}

impl Parser {
    fn parse_program(&mut self) -> Vec<Token> {
        Vec::new()
    }
}