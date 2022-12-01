#[derive(Debug)]
pub struct Elf {
    items: Vec<i32>,
    total: i32,
}

impl Elf {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            total: 0,
        }
    }

    pub fn add(&mut self, item: i32) {
        self.items.push(item);
        self.total += item;
    }

    pub fn get_total(&self) -> i32 {
        self.total
    }
}
