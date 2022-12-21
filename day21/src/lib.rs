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

    fn find_human_path(&self, name: &str) -> Option<Vec<String>> {
        if name == "humn" {
            return Some(vec![name.to_string()]);
        }
        match self.monkeys.get(name).unwrap() {
            Monkey::Num(_) => None,
            Monkey::Add(left, right)
            | Monkey::Sub(left, right)
            | Monkey::Mul(left, right)
            | Monkey::Div(left, right) => {
                if let Some(mut v) = self.find_human_path(left) {
                    v.push(name.to_string());
                    Some(v)
                } else if let Some(mut v) = self.find_human_path(right) {
                    v.push(name.to_string());
                    Some(v)
                } else {
                    None
                }
            }
        }
    }
    fn solve(&self, human_path: &mut Vec<String>, name: &str, equal_to: i64) -> i64 {
        if name == "humn" {
            return equal_to;
        }
        let monkey = self.monkeys.get(name).unwrap();
        let (left, right) = match monkey {
            Monkey::Num(_) => unreachable!(),
            Monkey::Add(left, right)
            | Monkey::Sub(left, right)
            | Monkey::Mul(left, right)
            | Monkey::Div(left, right) => (left, right),
        };
        let is_human_left = left == &human_path.pop().unwrap();
        let (to_solve, other) = if is_human_left {
            (left, self.get_answer(right))
        } else {
            (right, self.get_answer(left))
        };
        let sub_answer = match (monkey, is_human_left) {
            (Monkey::Num(_), _) => unreachable!(),
            (Monkey::Add(_, _), _) => equal_to - other,
            (Monkey::Sub(_, _), true) => equal_to + other,
            (Monkey::Sub(_, _), false) => other - equal_to,
            (Monkey::Mul(_, _), _) => equal_to / other,
            (Monkey::Div(_, _), true) => equal_to * other,
            (Monkey::Div(_, _), false) => other / equal_to,
        };
        self.solve(human_path, to_solve, sub_answer)
    }

    pub fn pt_2(&mut self) -> i64 {
        let mut human_path = self.find_human_path("root").unwrap();
        println!("{:?}", human_path);
        human_path.pop();
        let (left, right) = match self.monkeys.get("root").unwrap() {
            Monkey::Add(left, right)
            | Monkey::Sub(left, right)
            | Monkey::Mul(left, right)
            | Monkey::Div(left, right) => (left, right),
            Monkey::Num(_) => unreachable!(),
        };
        if left == &human_path.pop().unwrap() {
            let right_result = self.get_answer(right);
            self.solve(&mut human_path, left, right_result)
        } else {
            let left_result = self.get_answer(left);
            self.solve(&mut human_path, right, left_result)
        }
    }
}
