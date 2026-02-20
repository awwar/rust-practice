mod call;
mod r#if;
mod print;
mod r#return;
mod var;
mod expression;
mod procedure;
mod rand;
mod sum;
mod type_converter;

pub use crate::procedure::procedure::Procedure;
use std::collections::HashMap;
use std::string::ToString;

pub fn get_procedures() -> HashMap<&'static str, Box<dyn Procedure>> {
    let mut procedures = HashMap::<&'static str, Box<dyn Procedure>>::new();

    procedures.insert("CALL", Box::new(call::Call {}));
    procedures.insert("IF", Box::new(r#if::If {}));
    procedures.insert("PRINT", Box::new(print::Print {}));
    procedures.insert("RETURN", Box::new(r#return::Return {}));
    procedures.insert("VAR", Box::new(var::Var {}));
    procedures.insert("RAND", Box::new(rand::Rand {}));
    procedures.insert("SUM", Box::new(sum::Sum {}));
    procedures.insert("BOOL", Box::new(type_converter::TypeConverter {}));
    procedures.insert("FLOAT", Box::new(type_converter::TypeConverter {}));
    procedures.insert("STRING", Box::new(type_converter::TypeConverter {}));
    procedures.insert("VOID", Box::new(type_converter::TypeConverter {}));
    procedures.insert("+", Box::new(expression::Expression { op: |_: String, _: String| { "0".to_string() } }));
    procedures.insert("-", Box::new(expression::Expression { op: |_: String, _: String| { "0".to_string() } }));
    procedures.insert("/", Box::new(expression::Expression { op: |_: String, _: String| { "0".to_string() } }));
    procedures.insert("*", Box::new(expression::Expression { op: |_: String, _: String| { "0".to_string() } }));
    procedures.insert("^", Box::new(expression::Expression { op: |_: String, _: String| { "0".to_string() } }));
    procedures.insert("=", Box::new(expression::Expression { op: |l: String, r: String| { l.eq(&r).to_string() } }));
    procedures.insert("<", Box::new(expression::Expression { op: |_: String, _: String| { "true".parse().unwrap() } }));
    procedures.insert(">", Box::new(expression::Expression { op: |_: String, _: String| { "true".parse().unwrap() } }));

    return procedures;
}