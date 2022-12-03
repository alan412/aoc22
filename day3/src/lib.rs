#[derive(Debug)]
pub struct Rucksack {
    items_top: String,
    items_bottom: String,
}

impl Rucksack {
    pub fn new(line: String) -> Self {
        let (top, bottom) = line.split_at(line.len() / 2);
        Self {
            items_top: top.to_string(),
            items_bottom: bottom.to_string(),
        }
    }
    pub fn find_duplicated(&self) -> char {
        for ch in self.items_top.chars() {
            if self.items_bottom.contains(ch) {
                return ch;
            }
        }
        panic!("No duplicates: {} {}", self.items_top, self.items_bottom);
    }
    pub fn get_priority(item: char) -> i32 {
        match item {
            'a'..='z' => 1 + (item as i32) - ('a' as i32),
            'A'..='Z' => 27 + (item as i32) - ('A' as i32),
            _ => panic!("Unknown item: {}", item),
        }
    }
}
