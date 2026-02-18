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

pub fn get_procedures() -> HashMap<String, Box<dyn Procedure>> {
    let mut procedures = HashMap::<String, Box<dyn Procedure>>::new();

    procedures.insert("CALL".to_string(), Box::new(call::Call {}));
    procedures.insert("IF".to_string(), Box::new(r#if::If {}));
    procedures.insert("PRINT".to_string(), Box::new(print::Print {}));
    procedures.insert("RETURN".to_string(), Box::new(r#return::Return {}));
    procedures.insert("VAR".to_string(), Box::new(var::Var {}));
    procedures.insert("+".to_string(), Box::new(expression::Expression {}));
    procedures.insert("-".to_string(), Box::new(expression::Expression {}));
    procedures.insert("/".to_string(), Box::new(expression::Expression {}));
    procedures.insert("*".to_string(), Box::new(expression::Expression {}));
    procedures.insert("^".to_string(), Box::new(expression::Expression {}));
    procedures.insert("=".to_string(), Box::new(expression::Expression {}));
    procedures.insert("<".to_string(), Box::new(expression::Expression {}));
    procedures.insert(">".to_string(), Box::new(expression::Expression {}));

    return procedures;
}