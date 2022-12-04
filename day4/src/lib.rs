#[derive(Debug)]
pub struct CleaningAssignment {
    first: i32,
    second: i32,
}

impl CleaningAssignment {
    pub fn new(first: &str, second: &str) -> Self {
        Self {
            first: first.parse::<i32>().unwrap(),
            second: second.parse::<i32>().unwrap(),
        }
    }
    pub fn fully_contained(&self, other: &CleaningAssignment) -> bool {
        if self.first >= other.first && self.second <= other.second {
            return true;
        } else if other.first >= self.first && other.second <= self.second {
            return true;
        }
        false
    }
}
