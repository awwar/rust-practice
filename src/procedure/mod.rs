mod call;
mod r#if;
mod print;
mod r#return;
mod var;
mod expression;
mod procedure;

pub use crate::procedure::procedure::Procedure;
use std::collections::HashMap;
use std::string::ToString;

// pub const PROCEDURES: [(String, &dyn Procedure); 5] = [
//     ("CALL".to_string(), &call::Call {}),
//     ("IF".to_string(), &r#if::If {}),
//     ("PRINT".to_string(), &print::Print {}),
//     ("RETURN".to_string(), &r#return::Return {}),
//     ("VAR".to_string(), &var::Var {}),
// ];

pub fn get_procedures() -> HashMap<String, Box<dyn Procedure>> {
    let mut procedures = HashMap::<String, Box<dyn Procedure>>::new();

    procedures.insert("CALL".to_string(), Box::new(call::Call {}));
    procedures.insert("IF".to_string(), Box::new(r#if::If {}));
    procedures.insert("PRINT".to_string(), Box::new(print::Print {}));
    procedures.insert("RETURN".to_string(), Box::new(r#return::Return {}));
    procedures.insert("VAR".to_string(), Box::new(var::Var {}));

    return procedures;
}

pub const EXPRESSIONS: &[(&str, &dyn Procedure); 8] = &[
    ("+", &expression::Expression {}),
    ("-", &expression::Expression {}),
    ("/", &expression::Expression {}),
    ("*", &expression::Expression {}),
    ("^", &expression::Expression {}),
    ("=", &expression::Expression {}),
    ("<", &expression::Expression {}),
    (">", &expression::Expression {}),
];
