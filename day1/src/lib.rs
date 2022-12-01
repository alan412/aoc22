pub struct Elf {
    items: Vec<i32>,
}

impl Elf {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: i32) {
        self.items.push(item);
    }

    pub fn get_total(&self) -> i32 {
        let mut total = 0;

        for i in &self.items {
            total += i;
        }
        total
    }
}
