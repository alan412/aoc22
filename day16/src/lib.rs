use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list0,
    sequence::tuple,
};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Room {
    name: String,
    flow_rate: u32,
    connections_str: Vec<String>,
    connections_int: Vec<usize>,
}

#[derive(Debug)]
pub struct Cave {
    decoder: HashMap<String, usize>,
    rooms: Vec<Room>,
}

fn parse_valve_name(i: &str) -> nom::IResult<&str, &str> {
    take_while(|c: char| c.is_alphabetic())(i)
}

impl Room {
    pub fn new(name: String, flow_rate: u32, connections: Vec<&str>) -> Self {
        let mut str_connections: Vec<String> = Vec::new();
        for c in connections {
            str_connections.push(c.to_string());
        }
        Self {
            name,
            flow_rate,
            connections_str: str_connections,
            connections_int: Vec::new(),
        }
    }
}

impl Cave {
    pub fn new() -> Self {
        Self {
            rooms: Vec::new(),
            decoder: HashMap::new(),
        }
    }
    pub fn add_room(&mut self, line: &String) {
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
            .push(Room::new(room_name.to_string(), flow_rate, connections));
        self.decoder
            .insert(room_name.to_string(), self.rooms.len() - 1);
    }
    pub fn update_rooms(&mut self) {
        for room in self.rooms.iter_mut() {
            for name in room.connections_str.iter() {
                match self.decoder.get(name) {
                    Some(dest_num) => room.connections_int.push(*dest_num),
                    None => {}
                }
            }
        }
    }
    pub fn part_1(&mut self) -> u32 {
        let curr_room = match self.decoder.get("AA") {
            Some(val) => *val,
            None => 0,
        };
        let mut cache = HashMap::new();

        let mut path = Vec::new();
        self.max_flow(curr_room, &mut path, 30, &mut cache)
    }

    fn max_flow(
        &self,
        curr_room: usize,
        path: &mut Vec<usize>,
        time_left: i32,
        cache: &mut HashMap<(usize, Vec<usize>, i32), u32>,
    ) -> u32 {
        if time_left <= 0 {
            return 0;
        }
        if let Some(&result) = cache.get(&(curr_room, path.to_vec(), time_left)) {
            return result;
        }
        let mut best = u32::MIN;

        if self.rooms[curr_room].flow_rate > 0 && !path.contains(&curr_room) {
            for dest in self.rooms[curr_room].connections_int.iter() {
                path.push(curr_room);
                let sub_result = self.max_flow(*dest, path, time_left - 2, cache);
                best =
                    best.max(sub_result + self.rooms[curr_room].flow_rate * (time_left - 1) as u32);
                path.pop();
            }
        }
        for dest in self.rooms[curr_room].connections_int.iter() {
            let sub_result = self.max_flow(*dest, path, time_left - 1, cache);
            best = best.max(sub_result);
        }
        cache.insert((curr_room, path.to_vec(), time_left), best);
        best
    }
}
