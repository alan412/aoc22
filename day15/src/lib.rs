use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

#[derive(Debug)]
struct Sensor {
    pt: Point,
    nearest_beacon: Point,
    distance: i32,
}

#[derive(Debug)]
pub struct SensorNetwork {
    sensors: Vec<Sensor>,
    beacons: HashMap<Point, bool>,
    top_left: Point,
    bottom_right: Point,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn distance(&self, other: &Self) -> i32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i32
    }
}
impl Sensor {
    pub fn new(sensor_pt: Point, beacon_pt: Point) -> Self {
        Self {
            pt: sensor_pt,
            nearest_beacon: beacon_pt,
            distance: sensor_pt.distance(&beacon_pt),
        }
    }

    pub fn get_range_for_row(&self, row: i32) -> Option<RangeInclusive<i32>> {
        let diff_y = self.pt.y.abs_diff(row) as i32;
        if diff_y > self.distance {
            return None;
        }
        let max_diff_x = self.distance - diff_y;
        Some((self.pt.x - max_diff_x)..=(self.pt.x + max_diff_x))
    }
}

fn join_range(ranges: &[RangeInclusive<i32>]) -> (RangeInclusive<i32>, Vec<RangeInclusive<i32>>) {
    let mut indexes = (1..ranges.len()).collect::<HashSet<usize>>();
    let mut range = ranges[0].to_owned();
    loop {
        let mut indexes_to_remove: Vec<usize> = Vec::new();
        for index in indexes.iter() {
            let curr_range = &ranges[*index];
            if range.contains(curr_range.start()) {
                indexes_to_remove.push(*index);
                if !range.contains(curr_range.end()) {
                    let new_range = *range.start()..=*curr_range.end();
                    range = new_range;
                }
            } else if curr_range.contains(range.start()) {
                indexes_to_remove.push(*index);
                if !curr_range.contains(range.end()) {
                    let new_range = *curr_range.start()..=*range.end();
                    range = new_range;
                } else {
                    range = curr_range.to_owned();
                }
            }
        }
        if indexes_to_remove.is_empty() {
            let mut remainder = indexes
                .iter()
                .map(|i| ranges[*i].to_owned())
                .collect::<Vec<RangeInclusive<i32>>>();
            return if remainder.len() < 2 {
                (range, remainder)
            } else {
                remainder.push(range);
                join_range(&remainder)
            };
        } else {
            indexes_to_remove.iter().rev().for_each(|i| {
                indexes.remove(i);
            });
        }
    }
}

impl SensorNetwork {
    pub fn new() -> Self {
        Self {
            sensors: Vec::new(),
            beacons: HashMap::new(),
            top_left: Point::new(i32::MAX, i32::MAX),
            bottom_right: Point::new(0, 0),
        }
    }
    pub fn add_line(&mut self, sensor_x: i32, sensor_y: i32, beacon_x: i32, beacon_y: i32) {
        let sensor = Point::new(sensor_x, sensor_y);
        let beacon = Point::new(beacon_x, beacon_y);

        self.top_left.y = min(self.top_left.y, min(sensor_y, beacon_y));
        self.top_left.x = min(self.top_left.x, min(sensor_x, beacon_x));
        self.bottom_right.y = max(self.bottom_right.y, max(sensor_y, beacon_y));
        self.bottom_right.x = max(self.bottom_right.x, max(sensor_x, beacon_x));

        self.sensors.push(Sensor::new(sensor, beacon));
        self.beacons.insert(beacon, true);
    }

    pub fn cannot_be(&self, row_num: i32) -> usize {
        let mut result = self
            .sensors
            .iter()
            .filter_map(|sensor| sensor.get_range_for_row(row_num))
            .flatten()
            .collect::<HashSet<i32>>();
        self.sensors.iter().for_each(|sensor| {
            if sensor.nearest_beacon.y == row_num {
                result.remove(&sensor.nearest_beacon.x);
            }
        });
        result.len()
    }

    pub fn get_frequency(&self, max: i32) -> i64 {
        for row in 0..=max {
            let ranges = self
                .sensors
                .iter()
                .filter_map(|sensor| sensor.get_range_for_row(row))
                .collect::<Vec<RangeInclusive<i32>>>();
            let (range, remainder) = join_range(&ranges);

            if !remainder.is_empty() {
                ranges.iter().for_each(|r| {
                    println!("- {:?}", r);
                });
                let x = if range.end() < remainder.get(0).unwrap().start() {
                    range.end() + 1
                } else {
                    range.start() - 1
                } as i64;
                println!("({}. {})", x, row);
                return (x * 4_000_000) + row as i64;
            }
        }
        0
    }
}
