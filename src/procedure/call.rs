use crate::lexer::Token;
use crate::parser::{Node, Parser};
use crate::procedure::Procedure;

pub(crate) struct Call {}

impl Procedure for Call {
    fn parse(&self, token: Token, parser: &mut Parser) -> Result<Node, String> {
        // CALL #NAME () $RESULT
        let link = match parser.subparse_flow_link() {
            Ok(l) => l,
            Err(e) => return Err(e)
        };

        let args = match parser.subparse_list_in_bracers(None) {
            Ok(l) => l,
            Err(e) => return Err(e)
        };

        let variable = match parser.subparse_variable_name() {
            Ok(l) => l,
            Err(e) => return Err(e)
        };

        let params = vec![link, variable].into_iter().chain(args.into_iter()).collect::<Vec<Node>>();

        Ok(Node::new_operation(token.value, params, token.at))
    }
}


#[cfg(test)]
mod tests {
    use crate::procedure::PROCEDURES;

    #[test]
    fn test_convert_string_to_integer_when_ok() {
        for flag in PROCEDURES {
            println!("{}", flag.0);
        }
    }
}

