#[derive(Debug, Clone)]
pub struct CrateStack {
    crates: Vec<char>,
}

impl CrateStack {
    pub fn new() -> Self {
        Self { crates: Vec::new() }
    }

    pub fn add(&mut self, new_crate: char) {
        self.crates.insert(0, new_crate);
    }

    pub fn transfer(&mut self, other: &mut CrateStack, num_crates: i32) {
        for _ in 0..num_crates {
            self.crates.push(other.pop());
        }
    }
    pub fn transfer_part2(&mut self, other: &mut CrateStack, num_crates: i32) {
        let mut tmp_stack: CrateStack = CrateStack::new();
        for _ in 0..num_crates {
            tmp_stack.push(other.pop());
        }
        for _ in 0..num_crates {
            self.push(tmp_stack.pop());
        }
    }
    pub fn pop(&mut self) -> char {
        self.crates.pop().unwrap()
    }
    pub fn push(&mut self, new_element: char) {
        self.crates.push(new_element);
    }
    pub fn last(&self) -> Option<&char> {
        self.crates.last()
    }
}
