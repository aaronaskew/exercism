use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until1},
    character::complete::{self, multispace1, satisfy},
    combinator::recognize,
    multi::{many1, separated_list0, separated_list1},
    sequence::{preceded, terminated},
};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug)]
struct DictionaryEntry {
    name: String,
    body: Vec<String>,
    next: Option<Box<DictionaryEntry>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Debug)]
pub struct Forth {
    stack: Vec<Value>,
    /// Current (most-recent) DictionaryEntry
    dictionary_head: Option<Box<DictionaryEntry>>,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: vec![],
            dictionary_head: None,
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input
            .chars()
            .map(|ch| ch.to_ascii_lowercase())
            .collect::<String>();

        let commands = parse(&input);

        for command in commands {
            match command {
                Command::Definition(entry) => {
                    assert!(entry.len() > 1);

                    let name = entry[0].clone();
                    let body = entry[1..].to_vec();

                    if name.parse::<i32>().is_ok() {
                        return Err(Error::InvalidWord);
                    }

                    self.dictionary_push(name, body);
                }
                Command::Word(w) => match w.as_str() {
                    word if self.dictionary_get(word.to_string()).is_some() => {
                        let body = self.dictionary_get(word.to_string()).unwrap().clone();

                        for body_word in body {
                            self.eval(body_word.clone().as_str())?;
                        }
                    }
                    "+" => {
                        if let Some(rhs) = self.stack.pop()
                            && let Some(lhs) = self.stack.pop()
                        {
                            self.stack.push(lhs + rhs);
                        } else {
                            return Err(Error::StackUnderflow);
                        }
                    }
                    "-" => {
                        if let Some(rhs) = self.stack.pop()
                            && let Some(lhs) = self.stack.pop()
                        {
                            self.stack.push(lhs - rhs);
                        } else {
                            return Err(Error::StackUnderflow);
                        }
                    }
                    "*" => {
                        if let Some(rhs) = self.stack.pop()
                            && let Some(lhs) = self.stack.pop()
                        {
                            self.stack.push(lhs * rhs);
                        } else {
                            return Err(Error::StackUnderflow);
                        }
                    }
                    "/" => {
                        if let Some(rhs) = self.stack.pop()
                            && let Some(lhs) = self.stack.pop()
                        {
                            if rhs == 0 {
                                return Err(Error::DivisionByZero);
                            }

                            self.stack.push(lhs / rhs);
                        } else {
                            return Err(Error::StackUnderflow);
                        }
                    }
                    "dup" => {
                        if let Some(value) = self.stack.pop() {
                            self.stack.push(value);
                            self.stack.push(value);
                        } else {
                            return Err(Error::StackUnderflow);
                        }
                    }
                    "drop" => {
                        if self.stack.pop().is_none() {
                            return Err(Error::StackUnderflow);
                        }
                    }
                    "swap" => {
                        if let Some(first) = self.stack.pop()
                            && let Some(second) = self.stack.pop()
                        {
                            self.stack.push(first);
                            self.stack.push(second);
                        } else {
                            return Err(Error::StackUnderflow);
                        }
                    }
                    "over" => {
                        if let Some(first) = self.stack.pop()
                            && let Some(copy_me) = self.stack.pop()
                        {
                            self.stack.push(copy_me);
                            self.stack.push(first);
                            self.stack.push(copy_me);
                        } else {
                            return Err(Error::StackUnderflow);
                        }
                    }

                    _ => {
                        return Err(Error::UnknownWord);
                    }
                },
                Command::Value(value) => {
                    self.stack.push(value);
                }
            }
        }

        Ok(())
    }

    fn dictionary_push(&mut self, name: String, body: Vec<String>) {
        let new_entry = Box::new(DictionaryEntry {
            next: self.dictionary_head.take(),
            name,
            body,
        });

        self.dictionary_head = Some(new_entry);
    }

    fn dictionary_get(&self, name: String) -> Option<Vec<String>> {
        Self::dictionary_get_starting_at(&self.dictionary_head, name)
    }

    fn dictionary_get_starting_at(
        starting_entry: &Option<Box<DictionaryEntry>>,
        name: String,
    ) -> Option<Vec<String>> {
        let mut curr = starting_entry;

        while let Some(entry) = curr {
            if name == entry.name {
                let body = &entry.body;
                curr = &entry.next;

                // If all of the words in the entry body are not found
                // in the dictionary (i.e. they are all builtins), return
                // the entry body.
                if body.iter().all(|body_word| {
                    Self::dictionary_get_starting_at(curr, body_word.clone()).is_none()
                }) {
                    return Some(body.to_vec());

                // If some of the words still need to be expanded via the dictionary, recurse until all
                // words are expanded to builtins.
                } else {
                    return Some(
                        body.iter()
                            .flat_map(|body_word| {
                                if let Some(body) =
                                    Self::dictionary_get_starting_at(curr, body_word.clone())
                                {
                                    body.clone()
                                } else {
                                    vec![body_word.clone()]
                                }
                            })
                            .collect(),
                    );
                }
            } else {
                curr = &entry.next;
            }
        }

        None
    }
}

#[derive(Debug)]
enum Command {
    Definition(Vec<String>),
    Word(String),
    Value(Value),
}

fn parse(input: &str) -> Vec<Command> {
    separated_list1(multispace1, alt((definition, value, word)))
        .parse(input)
        .map(|(_, commands)| commands)
        .unwrap_or(Vec::new())
}

fn definition(input: &str) -> IResult<&str, Command> {
    terminated(
        preceded(tag(": "), separated_list0(multispace1, take_until1(" "))),
        tag(" ;"),
    )
    .parse(input)
    .map(|(input, commands)| {
        (
            input,
            Command::Definition(commands.iter().map(|s| s.to_string()).collect()),
        )
    })
}

fn value(input: &str) -> IResult<&str, Command> {
    complete::i32(input).map(|(input, value)| (input, Command::Value(value)))
}

fn word(input: &str) -> IResult<&str, Command> {
    recognize(many1(satisfy(|c| !c.is_whitespace())))
        .parse(input)
        .map(|(input, word)| (input, Command::Word(word.to_string())))
}
