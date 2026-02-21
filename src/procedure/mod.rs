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
use crate::program::Value;
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
    procedures.insert("BOOL", Box::new(type_converter::TypeConverter { op: Value::to_bool, opname: "BOOL".to_string() }));
    procedures.insert("FLOAT", Box::new(type_converter::TypeConverter { op: Value::to_float, opname: "FLOAT".to_string() }));
    procedures.insert("STRING", Box::new(type_converter::TypeConverter { op: Value::to_string, opname: "STRING".to_string() }));
    procedures.insert("VOID", Box::new(type_converter::TypeConverter { op: |_| { Value::Integer(0) }, opname: "VOID".to_string() }));
    procedures.insert("+", Box::new(expression::Expression { op: Value::add }));
    procedures.insert("-", Box::new(expression::Expression { op: Value::subtract }));
    procedures.insert("/", Box::new(expression::Expression { op: Value::divide }));
    procedures.insert("*", Box::new(expression::Expression { op: Value::multiply }));
    procedures.insert("^", Box::new(expression::Expression { op: Value::power }));
    procedures.insert("=", Box::new(expression::Expression { op: Value::eq }));
    procedures.insert("<", Box::new(expression::Expression { op: Value::less }));
    procedures.insert(">", Box::new(expression::Expression { op: Value::more }));

    procedures
}