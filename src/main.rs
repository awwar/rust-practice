#[derive(Copy, PartialEq, Eq, Clone, Debug)]
enum Token {
    Word,
    Number,
    Operator,
    Bracket,
    Space,
    String,
}

fn main() {
    let input = "1 + + log_2(45 + 5)";
    //let input = "1 + \"asda asd asd\" + log_2(45 + 5)";
    let chars: Vec<char> = input.chars().collect();

    let mut specs: Specs = Specs::new();

    let mut nodes: Vec<Node> = Vec::new();

    let mut pos: usize = 0;

    let mut buffer = String::new();

    loop {
        let mut char = '\0';
        if pos < chars.len() {
            char = chars[pos];
        }

        let candidate = specs.decide(char, buffer.clone());

        if candidate.is_none() {
            buffer.push(char);
            pos += 1;
        } else {
            nodes.push(Node::new(pos, candidate.unwrap().name, buffer.clone()));
            buffer = String::new();
            specs.reset();
        }

        if char == '\0' {
            break;
        }
    }

    nodes
        .iter()
        .for_each(|node| println!("{}", node.to_string()));
}

struct Node {
    token: Token,
    at: usize,
    value: String,
}

impl Node {
    fn new(pos: usize, name: Token, buffer: String) -> Node {
        Node {
            token: name,
            at: std::cmp::max(pos - buffer.len(), 0),
            value: buffer,
        }
    }
    fn to_string(&self) -> String {
        format!("{} {} {}", self.at, format!("{:?}", self.token), self.value)
    }
}

struct Specs(Vec<Spec>);

impl Specs {
    fn new() -> Self {
        Specs(Vec::from([
            Spec::new(Token::Space, |c, _| c.is_whitespace()),
            // 111 1 1.1 .1
            Spec::new(Token::Number, |c, _| c.is_numeric()),
            // aaa 1aa a1a a_1a
            Spec::new(Token::Word, |c, _| c.is_alphanumeric() || c.is_numeric() || c == '_'),
            // + - * / =
            Spec::new(Token::Operator, |c, _| vec!['+', '-', '*', '/'].contains(&c)),
            // ( ) [ ]
            Spec::new(Token::Bracket, |c, _| vec!['[', ']', '(', ')'].contains(&c)),
            // 111 1 1.1 .1
            Spec::new(Token::String, |c, b| (c == '"' && !b.ends_with('"')) || (c != '"' && b.starts_with('"'))),
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
            return None
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
    name: Token,
    when: fn(c: char, b: String) -> bool,
    accepted: bool,
}

impl Spec {
    fn new(name: Token, when: fn(c: char, b: String) -> bool) -> Spec {
        Spec { name, when, accepted: true }
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
