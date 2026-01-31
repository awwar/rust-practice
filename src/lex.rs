#[derive(Copy, PartialEq, Eq, Clone, Debug)]
enum TokenName {
    Whitespace,
    Word,
    Number,
    Operator,
    Comma,
    Bracket,
    String,
}

pub(crate) struct Lexer {
    chars: Vec<char>,
    pos: usize,
    specs: Specs,
}

impl Lexer {
    pub(crate) fn new(input: String) -> Lexer {
        Lexer{chars: input.chars().collect(), pos: 0, specs: Specs::new()}
    }
}
impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();

        loop {
            let char = *self.chars.get(self.pos).unwrap_or(&'\0');

            if buffer.len() == 0 && char == '\0' {
                return None;
            }

            let candidate = self.specs.decide(char, buffer.clone());

            if candidate.is_some() {
                self.specs.reset();
                let spec = candidate.unwrap();

                if spec.token_name == TokenName::Whitespace {
                    buffer.clear();
                    continue;
                }

                return Some(Token::new(spec.token_name, buffer.clone(), self.pos));
            }

            self.pos += 1;
            buffer.push(char);
        }
    }
}

pub(crate) struct Token {
    name: TokenName,
    at: usize,
    value: String,
}

impl Token {
    fn new(name: TokenName, value: String, pos: usize) -> Token {
        Token {
            name,
            at: std::cmp::max(pos - value.len(), 0),
            value,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.at, format!("{:?}", self.name), self.value)
    }
}

struct Specs(Vec<Spec>);

impl Specs {
    fn new() -> Self {
        Specs(Vec::from([
            Spec::new(TokenName::Whitespace, |c, _| c.is_whitespace() || c.is_control()),
            // 111 1 1.1 .1
            Spec::new(TokenName::Number, |c, b| c.is_numeric() || (c == '.' && !b.contains('.'))),
            // aaa 1aa a1a a_1a
            Spec::new(TokenName::Word, |c, _| { c.is_alphanumeric() || "#$_".contains(c) }),
            // + - * / =
            Spec::new(TokenName::Operator, |c, b| b.len() == 0 && "+-*/<>^=&|".contains(c)),
            // ( ) [ ]
            Spec::new(TokenName::Bracket, |c, b| b.len() == 0 && "[]()".contains(c)),
            Spec::new(TokenName::Comma, |c, b| b.len() == 0 && ",".contains(c)),
            // "foo bar baz"
            Spec::new(TokenName::String, |c, b| {
                !(b.len() > 1 && b.starts_with('"') && b.ends_with('"')) && (b.len() != 0 || c == '"')
            }),
        ]))
    }
    pub fn decide(&mut self, c: char, b: String) -> Option<Spec> {
        let mut candidate: Option<Spec> = None;
        let mut count = 0;

        for spec in self.0.iter_mut() {
            if candidate.is_none() && spec.accepted {
                candidate = Some(spec.clone());
            }
            spec.decide(c, b.clone());

            if !spec.accepted {
                continue;
            }
            count += 1;
        }

        if count > 0 {
            return None;
        }

        if candidate.is_none() || b.len() == 0 {
            panic!("got unexpected character \"{}\"", c);
        }

        candidate
    }
    pub fn reset(&mut self) {
        for spec in self.0.iter_mut() {
            spec.reset()
        }
    }
}

#[derive(Clone)]
struct Spec {
    token_name: TokenName,
    when: fn(c: char, b: String) -> bool,
    accepted: bool,
}

impl Spec {
    fn new(name: TokenName, when: fn(c: char, b: String) -> bool) -> Spec {
        Spec {
            token_name: name,
            when,
            accepted: true,
        }
    }
    fn decide(&mut self, c: char, b: String) {
        if !self.accepted {
            return;
        }

        self.accepted = (self.when)(c, b)
    }
    fn reset(&mut self) {
        self.accepted = true;
    }
}
