use std::collections::HashMap;

struct Node {
    value: i32,
    moved: bool,
}

pub struct Puzzle {
    original_list: Vec<Node>,
}

impl Puzzle {
    pub fn new() -> Puzzle {
        Self {
            original_list: Vec::new(),
        }
    }
    pub fn add_line(&mut self, line: &str) {
        self.original_list.push(Node {
            value: line.parse().unwrap(),
            moved: false,
        });
    }
    pub fn mixing(&mut self) {
        let len = self.original_list.len();
        let mut i = 0;
        let wrap = len as i32 - 1;

        while i < len {
            if self.original_list[i].moved {
                i += 1;
                continue;
            }

            let mut new_location = (i as i32 + self.original_list[i].value) % wrap;
            if new_location < 0 {
                new_location += wrap
            }
            let mut to_move = self.original_list.remove(i);
            to_move.moved = true;
            self.original_list.insert(new_location as usize, to_move);
        }
    }
    pub fn pt_1(&mut self) -> i32 {
        self.mixing();

        let pos0 = self
            .original_list
            .iter()
            .position(|e| e.value == 0)
            .expect("List should contain a 0");
        let n = self.original_list.len();

        self.original_list[(pos0 + 1000) % n].value
            + self.original_list[(pos0 + 2000) % n].value
            + self.original_list[(pos0 + 3000) % n].value
    }
    pub fn pt_2(&self) -> i32 {
        0
    }
}
