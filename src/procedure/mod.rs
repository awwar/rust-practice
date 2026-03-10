mod array;
mod expression;
mod print;
mod rand;
mod sum;
mod type_converter;

use crate::compiler::Compiler;
use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::program::Value;
use crate::vm::Stack;

pub trait Procedure {
    fn parse(&self, procedure: Node, variable: Node, args: Vec<Node>) -> Result<Node, String> {
        let mut params = vec![variable];
        params.extend(args);

        Ok(Node::new_operation(procedure.value, params, procedure.token_position))
    }
    fn compile(&self, sc: &mut Compiler, node: Node) -> Result<(), String> {
        sc.sub_compile(node)
    }
    fn execute(&self, _argc: usize, _stack: &mut Stack) -> Result<(), String> {
        panic!("procedure not implemented yet");
    }
}

pub fn get_procedures(name: &str) -> Box<dyn Procedure> {
    match name {
        "PRINT" => Box::new(print::Print {}),
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
        "-" => Box::new(expression::Expression { op: Value::subtract }),
        "/" => Box::new(expression::Expression { op: Value::divide }),
        "*" => Box::new(expression::Expression { op: Value::multiply }),
        "^" => Box::new(expression::Expression { op: Value::power }),
        "=" => Box::new(expression::Expression { op: Value::eq }),
        "<" => Box::new(expression::Expression { op: Value::less }),
        ">" => Box::new(expression::Expression { op: Value::more }),
        _ => panic!("Unknown procedure {name}"),
    }
}
