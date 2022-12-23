use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Debug)]
pub struct Puzzle {
    elves: HashSet<Point>,
    line_num: i32,
    order: VecDeque<Dir>,
}

impl Puzzle {
    pub fn new() -> Self {
        let mut order: VecDeque<Dir> = VecDeque::new();
        order.push_back(Dir::N);
        order.push_back(Dir::S);
        order.push_back(Dir::W);
        order.push_back(Dir::E);

        Self {
            elves: HashSet::new(),
            line_num: 0,
            order,
        }
    }
    pub fn add_line(&mut self, line: &str) {
        for (i, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    self.elves.insert(Point {
                        x: i as i32,
                        y: self.line_num,
                    });
                }
                '.' => {}
                _ => unreachable!(),
            };
        }
        self.line_num += 1;
    }
    fn propose_move(&self, elf: &Point, dir: &Dir) -> bool {
        match dir {
            Dir::N => {
                !self.elves.contains(&Point {
                    x: elf.x - 1,
                    y: elf.y - 1,
                }) && !self.elves.contains(&Point {
                    x: elf.x,
                    y: elf.y - 1,
                }) && !self.elves.contains(&Point {
                    x: elf.x + 1,
                    y: elf.y - 1,
                })
            }
            Dir::W => {
                !self.elves.contains(&Point {
                    x: elf.x - 1,
                    y: elf.y - 1,
                }) && !self.elves.contains(&Point {
                    x: elf.x - 1,
                    y: elf.y,
                }) && !self.elves.contains(&Point {
                    x: elf.x - 1,
                    y: elf.y + 1,
                })
            }
            Dir::S => {
                !self.elves.contains(&Point {
                    x: elf.x - 1,
                    y: elf.y + 1,
                }) && !self.elves.contains(&Point {
                    x: elf.x,
                    y: elf.y + 1,
                }) && !self.elves.contains(&Point {
                    x: elf.x + 1,
                    y: elf.y + 1,
                })
            }
            Dir::E => {
                !self.elves.contains(&Point {
                    x: elf.x + 1,
                    y: elf.y - 1,
                }) && !self.elves.contains(&Point {
                    x: elf.x + 1,
                    y: elf.y,
                }) && !self.elves.contains(&Point {
                    x: elf.x + 1,
                    y: elf.y + 1,
                })
            }
        }
    }
    fn empty_around(&self, elf: &Point) -> bool {
        for diff_y in -1..=1 {
            for diff_x in -1..=1 {
                if !(diff_x == 0 && diff_y == 0)
                    && self.elves.contains(&Point {
                        x: elf.x + diff_x,
                        y: elf.y + diff_y,
                    })
                {
                    return false;
                }
            }
        }
        true
    }

    fn round(&mut self) -> bool {
        let mut desired_move: HashMap<Point, Vec<Point>> = HashMap::new();
        let mut moved = false;

        for elf in self.elves.iter() {
            if self.empty_around(elf) {
                continue;
            }
            for dir in self.order.iter() {
                if self.propose_move(elf, dir) {
                    match dir {
                        Dir::N => desired_move
                            .entry(Point {
                                x: elf.x,
                                y: elf.y - 1,
                            })
                            .or_default()
                            .push(*elf),
                        Dir::E => desired_move
                            .entry(Point {
                                x: elf.x + 1,
                                y: elf.y,
                            })
                            .or_default()
                            .push(*elf),
                        Dir::S => desired_move
                            .entry(Point {
                                x: elf.x,
                                y: elf.y + 1,
                            })
                            .or_default()
                            .push(*elf),
                        Dir::W => desired_move
                            .entry(Point {
                                x: elf.x - 1,
                                y: elf.y,
                            })
                            .or_default()
                            .push(*elf),
                    }
                    break;
                } else {
                    //println!("-No move...");
                }
            }
        }
        // make moves
        for (space, list_elves) in desired_move.iter() {
            if list_elves.len() == 1 {
                moved = true;
                self.elves.remove(&list_elves[0]);
                self.elves.insert(*space);
            }
        }
        // change order of looking
        self.order.rotate_left(1);
        moved
    }

    fn score(&self) -> i32 {
        let mut largest_x = i32::MIN;
        let mut smallest_x = i32::MAX;
        let mut largest_y = i32::MIN;
        let mut smallest_y = i32::MAX;
        for elf in self.elves.iter() {
            largest_x = largest_x.max(elf.x);
            smallest_x = smallest_x.min(elf.x);
            largest_y = largest_y.max(elf.y);
            smallest_y = smallest_y.min(elf.y);
        }
        let mut answer = 0;
        for x in smallest_x..=largest_x {
            for y in smallest_y..=largest_y {
                if !self.elves.contains(&Point { x, y }) {
                    answer += 1;
                }
            }
        }
        answer
    }
    fn display(&self) {
        let mut largest_x = 10;
        let mut smallest_x = -3;
        let mut largest_y = 9;
        let mut smallest_y = -2;
        for elf in self.elves.iter() {
            largest_x = largest_x.max(elf.x);
            smallest_x = smallest_x.min(elf.x);
            largest_y = largest_y.max(elf.y);
            smallest_y = smallest_y.min(elf.y);
        }
        for y in smallest_y..=largest_y {
            let mut line: String = "".to_string();
            for x in smallest_x..=largest_x {
                if self.elves.contains(&Point { x, y }) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            println!("{}", line);
        }
    }
    pub fn pt_1(&mut self) -> i32 {
        self.display();

        for round in 0..10 {
            let moved = self.round();
            println!("Round {}", round + 1);
            //self.display();
        }
        self.score()
    }
    pub fn pt_2(&self) -> i32 {
        0
    }
}
