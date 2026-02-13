#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum TokenName {
    Whitespace,
    Word,
    Number,
    Operator,
    Comma,
    Bracket,
    String,
}

#[derive(Clone)]
pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new(input: String) -> Result<TokenStream, String> {
        let chars: Vec<char> = input.chars().collect();
        let mut buffer = String::new();
        let mut last_char_idx: usize = 0;
        let mut specs: Specs = Specs::new();
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let char = *chars.get(last_char_idx).unwrap_or(&'\0');

            if buffer.len() == 0 && char == '\0' {
                break;
            }

            let candidate = specs.decide(char, buffer.clone());

            if candidate.is_some() {
                specs.reset();
                let spec = candidate.unwrap();

                let token = Token::new(spec.token_name, buffer.clone(), last_char_idx);

                buffer.clear();

                if token.name != TokenName::Whitespace {
                    tokens.push(token);
                }

                continue;
            }

            last_char_idx += 1;
            buffer.push(char);
        }

        Ok(TokenStream { tokens })
    }
    pub fn get(&mut self, i: usize) -> Option<Token> {
        let candidate = self.tokens.get(i);

        if candidate.is_some() {
            return Some(candidate.unwrap().clone());
        }

        None
    }
    pub fn search_idx_of_closed_bracer(&mut self, mut current_position: usize) -> Option<usize> {
        let mut counts = 0;

        loop {
            let candidate = match self.tokens.get(current_position) {
                Some(token) => token,
                None => break,
            };

            match candidate.value.as_str() {
                "(" => counts+=1,
                ")" => counts-=1,
                _ => {}
            }

            if counts == 0 {
                return Some(current_position);
            }

            current_position+=1
        }

        None
    }
}

#[derive(Clone)]
pub struct Token {
    pub(crate) name: TokenName,
    pub at: usize,
    pub value: String,
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
    pub fn starts_with(&self, s: &str) -> bool {
        self.value.starts_with(s)
    }
}

struct Specs(Vec<Spec>);

impl Specs {
    fn new() -> Self {
        Specs(Vec::from([
            Spec::new(TokenName::Whitespace, |c, _| c.is_whitespace() || c.is_control()),
            // 111 1 1.1 .1
            Spec::new(TokenName::Number, |c, b| c.is_numeric() || (c == '.' && !b.contains('.')) || (c == '-' && b.len() == 0)),
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
