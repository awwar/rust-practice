use crate::lex::Lexer;

#[allow(dead_code)]
struct Parser {
    first_position: usize,
    last_position: usize,
    current_position: usize,

    lexer: Lexer,
}