use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list0,
    sequence::tuple,
};
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[derive(Debug)]
struct Room<'a> {
    flow_rate: u32,
    connections: HashSet<&'a str>,
}

#[derive(Debug)]
pub struct Cave<'a> {
    rooms: HashMap<&'a str, Room<'a>>,
}

#[derive(PartialEq, Eq)]
struct Node<'a> {
    cost: u32,
    curr: &'a str,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct State<'a> {
    opened: BTreeSet<&'a str>,
    curr: &'a str,
    elapsed: u32,
    relieved: u32,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_valve_name(i: &str) -> nom::IResult<&str, &str> {
    take_while(|c: char| c.is_alphabetic())(i)
}

impl<'a> Room<'a> {
    pub fn new(flow_rate: u32, connections: Vec<&'a str>) -> Self {
        let set: HashSet<&str> = connections.iter().map(|str| *str).collect();
        Self {
            flow_rate,
            connections: set,
        }
    }
}

impl<'a> Cave<'a> {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }
    pub fn add_room(&mut self, line: &'a str) {
        let mut parse_valve = tuple((
            tag("Valve "),
            parse_valve_name,
            tag(" has flow rate="),
            map_res(digit1, u32::from_str),
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list0(tag(", "), parse_valve_name),
        ));
        let result = parse_valve(line).unwrap();
        let (_, room_name, _, flow_rate, _, connections) = result.1;

        self.rooms
            .insert(room_name, Room::new(flow_rate, connections));
    }

    fn min_cost(&self, from: &str, to: &str) -> u32 {
        let mut pq = BinaryHeap::new();
        let mut seen = HashSet::new();

        pq.push(Node {
            cost: 0,
            curr: from,
        });

        while let Some(Node { cost, curr }) = pq.pop() {
            if curr == to {
                return cost;
            }
            for neighbor in self.rooms[curr].connections.iter() {
                if seen.insert(neighbor) {
                    pq.push(Node {
                        cost: cost + 1,
                        curr: neighbor,
                    });
                }
            }
        }
        u32::MAX
    }
    fn min_distances(&self) -> HashMap<(&'a str, &'a str), u32> {
        self.rooms
            .iter()
            .filter(|(_, room)| room.flow_rate > 0)
            .map(|(&name, _)| name)
            .tuple_combinations()
            .fold(HashMap::new(), |mut acc, (name1, name2)| {
                acc.entry(("AA", name1))
                    .or_insert_with(|| self.min_cost("AA", name1));
                acc.entry(("AA", name2))
                    .or_insert_with(|| self.min_cost("AA", name2));

                let dist = self.min_cost(name1, name2);
                acc.insert((name1, name2), dist);
                acc.insert((name2, name1), dist);
                acc
            })
    }
    fn wait_until_end(
        &self,
        max_time: u32,
        elapsed: u32,
        relieved: u32,
        opened: &BTreeSet<&str>,
    ) -> u32 {
        let time_left = max_time - elapsed;
        let relieved_per_min: u32 = opened.iter().map(|name| self.rooms[name].flow_rate).sum();
        relieved + (relieved_per_min * time_left)
    }

    pub fn part_1(&mut self) -> u32 {
        let dist_map = self.min_distances();
        let flowing: HashSet<_> = self
            .rooms
            .iter()
            .filter(|(_, valve)| valve.flow_rate > 0)
            .map(|(&name, _)| name)
            .collect();

        let mut max_relieved = 0;
        let mut q = VecDeque::new();
        let mut seen = HashSet::new();

        q.push_back(State {
            curr: "AA",
            opened: BTreeSet::new(),
            elapsed: 0,
            relieved: 0,
        });
        seen.insert((BTreeSet::new(), 0, 0));

        while let Some(State {
            opened,
            curr,
            elapsed,
            relieved,
        }) = q.pop_front()
        {
            if opened.len() == flowing.len() || elapsed >= 30 {
                let relieved_at_end = self.wait_until_end(30, elapsed, relieved, &opened);
                max_relieved = max_relieved.max(relieved_at_end);
                continue;
            }
            let unopened = flowing.iter().filter(|name| !opened.contains(*name));

            for dest in unopened {
                let cost = dist_map[&(curr, *dest)] + 1;
                let new_elapsed = elapsed + cost;

                if new_elapsed >= 30 {
                    let relieved_at_end = self.wait_until_end(30, elapsed, relieved, &opened);
                    max_relieved = max_relieved.max(relieved_at_end);
                    continue;
                }

                let relieved_per_min: u32 =
                    opened.iter().map(|name| &self.rooms[name].flow_rate).sum();
                let new_relieved = relieved + (relieved_per_min * cost);
                let mut new_opened = opened.clone();
                new_opened.insert(dest);

                if seen.insert((new_opened.clone(), new_elapsed, new_relieved)) {
                    q.push_back(State {
                        opened: new_opened,
                        curr: dest,
                        elapsed: new_elapsed,
                        relieved: new_relieved,
                    });
                }
            }
        }
        max_relieved
    }
    pub fn part_2(&mut self) -> u32 {
        0
    }
}
