use nom::{
    branch::alt, character::complete, combinator::map, multi::separated_list0, sequence::delimited,
    Finish, IResult,
};
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq)]
pub enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl Packet {
    pub fn new(input: &str) -> Self {
        fn parse(line: &str) -> IResult<&str, Packet> {
            alt((
                delimited(
                    complete::char('['),
                    map(separated_list0(complete::char(','), parse), Packet::List),
                    complete::char(']'),
                ),
                map(complete::u32, Packet::Int),
            ))(line)
        }
        let result = parse(input);
        result.unwrap().1
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.partial_cmp(r),
            (Packet::List(l), Packet::List(r)) => l.partial_cmp(r),
            (l, Packet::List(r)) => match &**r {
                [r, ..] if l != r => l.partial_cmp(r),
                _ => 1usize.partial_cmp(&r.len()),
            },
            (Packet::List(_), Packet::Int(_)) => other.partial_cmp(self).map(Ordering::reverse),
        }
    }
}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
