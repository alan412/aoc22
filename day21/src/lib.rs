use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, one_of, space0},
    combinator::map,
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
enum Monkey {
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Num(i64),
}

pub struct Puzzle {
    monkeys: HashMap<String, Monkey>,
}

fn parse_operation(input: &str) -> IResult<&str, Monkey> {
    alt((
        map(digit1, |s: &str| Monkey::Num(s.parse().unwrap())),
        map(
            tuple((alpha1, space0, one_of("+-*/"), space0, alpha1)),
            |t: (&str, &str, char, &str, &str)| match t.2 {
                '+' => Monkey::Add(t.0.to_string(), t.4.to_string()),
                '-' => Monkey::Sub(t.0.to_string(), t.4.to_string()),
                '*' => Monkey::Mul(t.0.to_string(), t.4.to_string()),
                '/' => Monkey::Div(t.0.to_string(), t.4.to_string()),
                _ => unreachable!(),
            },
        ),
    ))(input)
}
fn parser(input: &str) -> (&str, Monkey) {
    let result: IResult<&str, (&str, &str, Monkey)> =
        tuple((alpha1, tag(": "), parse_operation))(input);

    let unwrapped = result.unwrap().1;
    return (unwrapped.0, unwrapped.2);
}

impl Puzzle {
    pub fn new() -> Puzzle {
        Self {
            monkeys: HashMap::new(),
        }
    }
    pub fn add_line(&mut self, line: &str) {
        let (name, data) = parser(line);
        self.monkeys.insert(name.to_string(), data);
    }
    fn get_answer(&self, name: &str) -> i64 {
        match self.monkeys.get(name).unwrap() {
            Monkey::Add(x, y) => self.get_answer(x) + self.get_answer(y),
            Monkey::Sub(x, y) => self.get_answer(x) - self.get_answer(y),
            Monkey::Mul(x, y) => self.get_answer(x) * self.get_answer(y),
            Monkey::Div(x, y) => self.get_answer(x) / self.get_answer(y),
            Monkey::Num(x) => *x,
        }
    }

    pub fn pt_1(&self) -> i64 {
        self.get_answer("root")
    }
    pub fn pt_2(&self) -> i64 {
        0
    }
}
