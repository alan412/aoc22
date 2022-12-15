use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug)]
struct Sensor{
  pt : Point,
  nearest_beacon : Point,
  distance : i32,
}

#[derive(Debug)]
pub struct SensorNetwork {
    sensors : Vec<Sensor>,
    beacons : HashMap<Point, bool>,
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
    pub fn distance(&self, other : &Self) -> i32{
      (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
impl Sensor{
  pub fn new(sensor_pt : Point, beacon_pt : Point) -> Self{
    Self {
      pt : sensor_pt,
      nearest_beacon : beacon_pt,
      distance : sensor_pt.distance(&beacon_pt),
    }
  }
  pub fn covered(&self, pt : &Point) -> bool{
    if self.pt.distance(pt) > self.distance{
      false
    }else{
      true
    }
  }
}

impl SensorNetwork{
  pub fn new() -> Self{
    Self {
      sensors : Vec::new(),
      beacons : HashMap::new(),
      top_left: Point::new(i32::MAX, i32::MAX),
      bottom_right: Point::new(0, 0),
    }
  }
  pub fn add_line(&mut self, sensor_x : i32, sensor_y : i32, beacon_x : i32, beacon_y : i32) {
    let sensor = Point::new(sensor_x, sensor_y);
    let beacon = Point::new(beacon_x, beacon_y);
    
    self.top_left.y = min(self.top_left.y, min(sensor_y, beacon_y));
    self.top_left.x = min(self.top_left.x, min(sensor_x, beacon_x));
    self.bottom_right.y = max(self.bottom_right.y, max(sensor_y, beacon_y));
    self.bottom_right.x = max(self.bottom_right.x, max(sensor_x, beacon_x));

    self.sensors.push(Sensor::new(sensor, beacon));
    self.beacons.insert(beacon, true);
  }
  
  pub fn cannot_be(&self, row_num : i32) -> i32{
    let mut greatest_distance = 0;
    for sensor in &self.sensors{
      greatest_distance = max(greatest_distance, sensor.distance);
    }
    let mut total = 0;
    for x in (self.top_left.x - greatest_distance)..(self.bottom_right.x + greatest_distance){
      let pt = Point::new(x, row_num);
      match self.beacons.get(&pt){
        None => {
          for sensor in &self.sensors{
            if sensor.covered(&pt){
              //println!("Covered at {:?}", pt);
              total += 1;
              break;
            }
          }      
        }
        Some(_) => {}
      }
    }
    total
  }



}