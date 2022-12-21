use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i64,
    index: RefCell<usize>,
}

pub struct Puzzle {
    order: Vec<Rc<Node>>,
    list: Vec<Rc<Node>>,
}

const KEY: i64 = 811589153;

impl Puzzle {
    pub fn new() -> Puzzle {
        Self {
            order: Vec::new(),
            list: Vec::new(),
        }
    }
    pub fn add_line(&mut self, line: &str) {
        let index = self.order.len();
        let value: i64 = line.parse().unwrap();
        let node = Rc::new(Node {
            value: value * KEY,
            index: RefCell::new(index),
        });
        self.order.push(node.clone());
        self.list.push(node);
    }

    fn mix(order: &Vec<Rc<Node>>, list: &mut Vec<Rc<Node>>) {
        let wrap = order.len() as i64 - 1;
        for node in order {
            let mut old_i = node.index.borrow_mut();
            let mut new_i = (*old_i as i64 + node.value) % wrap;
            if new_i < 0 {
                new_i += wrap;
            }
            let new_i = new_i as usize;

            if new_i < *old_i {
                for i in new_i..*old_i {
                    list[i].index.replace_with(|&mut old| old + 1);
                }
            } else if *old_i < new_i {
                for i in (*old_i + 1)..=new_i {
                    list[i].index.replace_with(|&mut old| old - 1);
                }
            }

            let to_move = list.remove(*old_i);
            *old_i = new_i;
            list.insert(new_i as usize, to_move);
        }
    }

    fn score(list: &Vec<Rc<Node>>) -> i64 {
        let pos0 = list
            .iter()
            .position(|e| e.value == 0)
            .expect("List should contain a 0");
        let n = list.len();

        list[(pos0 + 1000) % n].value
            + list[(pos0 + 2000) % n].value
            + list[(pos0 + 3000) % n].value
    }

    pub fn pt_1(&mut self) -> i32 {
        0
    }
    pub fn pt_2(&mut self) -> i64 {
        for _ in 0..10 {
            Self::mix(&self.order, &mut self.list);
        }
        Self::score(&self.list)
    }
}
