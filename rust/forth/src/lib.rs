use std::collections::HashMap;
use std::rc::Rc;

pub type Value = i64;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    defns: HashMap<String, Rc<Vec<Token>>>,
    mode: Mode,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Interpret,
    Name,
    Compile,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Token {
    Number(Value),
    Word(String),
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            defns: HashMap::new(),
            mode: Mode::Interpret,
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let code = Forth::parse(input);
        self.eval_code(&code)
    }

    fn parse(input: &str) -> Vec<Token> {
        input.split(|x: char| x.is_whitespace() || x.is_control())
            .map(|x| {
                Value::from_str_radix(x, 10)
                    .map(Token::Number)
                    .unwrap_or_else(|_| Token::Word(x.into()))
            })
            .collect()
    }

    fn eval_code(&mut self, code: &[Token]) -> ForthResult {
        let mut name = &"".into();
        let mut defn = Vec::new();
        for word in code {
            match self.mode {
                Mode::Name => {
                    match *word {
                        Token::Word(ref new_name) => {
                            name = new_name;
                            self.mode = Mode::Compile;
                        }
                        _ => return Err(Error::InvalidWord),
                    }
                }
                Mode::Compile => {
                    match *word {
                        Token::Word(ref text) if &text[..] == ";" => {
                            self.mode = Mode::Interpret;
                            self.defns.insert(name.to_uppercase(), Rc::new(defn));
                            defn = Vec::new();
                        }
                        ref instr => defn.push(instr.clone()),
                    }
                }
                Mode::Interpret => try!(self.interpret(word)),
            }
        }

        match self.mode {
            Mode::Interpret => Ok(()),
            _ => Err(Error::InvalidWord),
        }
    }

    fn interpret(&mut self, item: &Token) -> ForthResult {
        macro_rules! shuffle {
            ($stack:expr ; $($a:ident),* => $($b:expr),*) => {{
                __generate_lets!($stack; $($a),*);
                if let ($(Some($a)),*,) = ($($a),*,) {
                    $($stack.push($b));*
                } else {
                    return Err(Error::StackUnderflow);
                }
            }}
        }

        macro_rules! __generate_lets {
            ($stack:expr ; $a:ident) => { let $a = $stack.pop(); };
            ($stack:expr ; $a:ident, $($b:ident),* ) => {
                __generate_lets!($stack; $($b),*);
                let $a = $stack.pop();
            }
        }

        match *item {
            Token::Number(n) => self.stack.push(n),
            Token::Word(ref w) => {
                match &w.to_uppercase()[..] {
                    w if self.defns.contains_key(w) => {
                        let code = self.defns[w].clone();
                        try!(self.eval_code(&*code));
                    }
                    ":" => self.mode = Mode::Name,
                    "+" => shuffle!(self.stack; a, b => a + b),
                    "-" => shuffle!(self.stack; a, b => a - b),
                    "*" => shuffle!(self.stack; a, b => a * b),
                    "/" => {
                        shuffle!(self.stack; a, b => {
                            if b == 0 { return Err(Error::DivisionByZero) } else { a / b }
                        })
                    }
                    "DUP" => shuffle!(self.stack; a => a, a),
                    "DROP" => shuffle!(self.stack; _a => ),
                    "SWAP" => shuffle!(self.stack; a, b => b, a),
                    "OVER" => shuffle!(self.stack; a, b => a, b, a),
                    _ => return Err(Error::UnknownWord),
                }
            }
        }
        Ok(())
    }
}
