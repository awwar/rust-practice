mod array;
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
    match name {
        "CALL" => Box::new(call::Call {}),
        "IF" => Box::new(r#if::If {}),
        "PRINT" => Box::new(print::Print {}),
        "RETURN" => Box::new(r#return::Return {}),
        "VAR" => Box::new(var::Var {}),
        "RAND" => Box::new(rand::Rand::new()),
        "SUM" => Box::new(sum::Sum {}),
        "BOOL" => Box::new(type_converter::TypeConverter { op: Value::to_bool }),
        "FILL_RANDOM" => Box::new(array::FillRandom::new()),
        "AT" => Box::new(array::At {}),
        "FLOAT" => Box::new(type_converter::TypeConverter {
            op: Value::to_float,
        }),
        "STRING" => Box::new(type_converter::TypeConverter {
            op: Value::to_string,
        }),
        "INT" => Box::new(type_converter::TypeConverter {
            op: Value::to_integer,
        }),
        "ARRAY" => Box::new(type_converter::TypeConverter {
            op: |l: &Value| {
                Value::Array(match l {
                    Value::Integer(_) => Vec::<Value>::new(),
                    Value::Float(_) => Vec::<Value>::new(),
                    Value::Boolean(_) => Vec::<Value>::new(),
                    Value::String(_) => Vec::<Value>::new(),
                    Value::Array(_) => Vec::<Value>::new(),
                    _ => {panic!("unable to create array of {}", l.repr())}
                })
            }
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
        _ => panic!("Unknown procedure {name}"),
    }
}
