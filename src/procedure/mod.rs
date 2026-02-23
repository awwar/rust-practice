mod call;
mod expression;
mod r#if;
mod print;
mod procedure;
mod rand;
mod r#return;
mod sum;
mod type_converter;
mod var;

pub use crate::procedure::procedure::Procedure;
use crate::program::Value;

pub fn get_procedures(name: &str) -> Box<dyn Procedure> {
    return match name {
        "CALL" => Box::new(call::Call {}),
        "IF" => Box::new(r#if::If {}),
        "PRINT" => Box::new(print::Print {}),
        "RETURN" => Box::new(r#return::Return {}),
        "VAR" => Box::new(var::Var {}),
        "RAND" => Box::new(rand::Rand::new()),
        "SUM" => Box::new(sum::Sum {}),
        "BOOL" => Box::new(type_converter::TypeConverter { op: Value::to_bool }),
        "FLOAT" => Box::new(type_converter::TypeConverter {
            op: Value::to_float,
        }),
        "STRING" => Box::new(type_converter::TypeConverter {
            op: Value::to_string,
        }),
        "VOID" => Box::new(type_converter::TypeConverter {
            op: |_| Value::Integer(0),
        }),
        "+" => Box::new(expression::Expression { op: Value::add }),
        "-" => Box::new(expression::Expression {
            op: Value::subtract,
        }),
        "/" => Box::new(expression::Expression { op: Value::divide }),
        "*" => Box::new(expression::Expression {
            op: Value::multiply,
        }),
        "^" => Box::new(expression::Expression { op: Value::power }),
        "=" => Box::new(expression::Expression { op: Value::eq }),
        "<" => Box::new(expression::Expression { op: Value::less }),
        ">" => Box::new(expression::Expression { op: Value::more }),
        _ => panic!("Unknown procedure {}", name),
    };
}
