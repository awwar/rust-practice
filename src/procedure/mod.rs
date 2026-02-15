mod call;
mod r#if;
mod print;
mod r#return;
mod var;
mod expression;
mod procedure;

pub use crate::procedure::procedure::Procedure;

pub const PROCEDURES: &[(&str, &'static dyn Procedure); 13] = &[
    ("CALL", &call::Call {}),
    ("IF", &r#if::If {}),
    ("PRINT", &print::Print {}),
    ("RETURN", &r#return::Return {}),
    ("VAR", &var::Var {}),
    ("+", &expression::Expression {}),
    ("-", &expression::Expression {}),
    ("/", &expression::Expression {}),
    ("*", &expression::Expression {}),
    ("^", &expression::Expression {}),
    ("=", &expression::Expression {}),
    ("<", &expression::Expression {}),
    (">", &expression::Expression {}),
];
