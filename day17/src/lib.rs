use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug)]
pub struct Cave {
    lines: Vec<u8>,
    jets: Vec<char>,
    width: usize,
}
#[derive(Debug, Copy, Clone)]
enum Shape {
    Horiz,
    Cross,
    LShape,
    Vert,
    Square,
}
impl Shape {
    pub fn get_max_height(&self) -> usize {
        match self {
            Shape::Horiz => 1,
            Shape::Cross => 3,
            Shape::LShape => 3,
            Shape::Vert => 4,
            Shape::Square => 2,
        }
    }
    pub fn get_max_width(&self) -> usize {
        match self {
            Shape::Horiz => 4,
            Shape::Cross => 3,
            Shape::LShape => 3,
            Shape::Vert => 1,
            Shape::Square => 2,
        }
    }
    pub fn get_bits(&self) -> u16 {
        match self {
            Shape::Horiz => 0b1111_0000_0000_0000,
            Shape::Cross => 0b0100_1110_0100_0000,
            Shape::LShape => 0b0010_0010_1110_0000,
            Shape::Vert => 0b1000_1000_1000_1000,
            Shape::Square => 0b1100_1100_0000_0000,
        }
    }
    pub fn get_bits_row(&self, row: usize) -> u8 {
        let place_bits = self.get_bits();
        match row {
            1 => ((place_bits & 0b1111_0000_0000_0000) >> 12)
                .try_into()
                .unwrap(),
            2 => ((place_bits & 0b0000_1111_0000_0000) >> 8)
                .try_into()
                .unwrap(),
            3 => ((place_bits & 0b0000_0000_1111_0000) >> 4)
                .try_into()
                .unwrap(),
            4 => (place_bits & 0b0000_0000_0000_1111).try_into().unwrap(),
            _ => panic!("Shape only has 4 rows"),
        }
    }
}

impl Cave {
    pub fn new(str: String) -> Self {
        Self {
            lines: Vec::new(),
            jets: str.as_str().chars().collect(),
            width: 7,
        }
    }
    fn get_shape(rock_num: usize) -> Shape {
        match rock_num % 5 {
            0 => Shape::Horiz,
            1 => Shape::Cross,
            2 => Shape::LShape,
            3 => Shape::Vert,
            4 => Shape::Square,
            _ => panic!("Shouldn't get here"),
        }
    }
    fn get_shifted_line(&self, row: usize, left: usize) -> u16 {
        if row >= self.lines.len() {
            0x00
        } else {
            let result: u8 = match left {
                0 => (self.lines[row] & 0b0111_1000) >> 3,
                1 => (self.lines[row] & 0b0011_1100) >> 2,
                2 => (self.lines[row] & 0b0001_1110) >> 1,
                3 => self.lines[row] & 0b0000_1111,
                4 => (self.lines[row] & 0b0000_0111) << 1,
                5 => (self.lines[row] & 0b0000_0011) << 2,
                6 => (self.lines[row] & 0b0000_0001) << 3,
                _ => panic!("Unknown left"),
            };
            (result & 0x0f) as u16
        }
    }

    fn get_u16(&self, top: usize, left: usize) -> u16 {
        let mut row: Vec<u16> = vec![0; 4];

        row[0] = self.get_shifted_line(top, left);
        if top > 0 {
            row[1] = self.get_shifted_line(top - 1, left);
        }
        if top > 1 {
            row[2] = self.get_shifted_line(top - 2, left);
        }
        if top > 2 {
            row[3] = self.get_shifted_line(top - 3, left);
        }
        return (row[0] << 12) | (row[1] << 8) | (row[2] << 4) | (row[3]);
    }

    fn can_move(&self, shape: Shape, new_rock_top: i32, new_rock_left: i32) -> bool {
        if (new_rock_left < 0)
            || (new_rock_top < 0)
            || (new_rock_left + shape.get_max_width() as i32 > self.width as i32)
        {
            false
        } else {
            (shape.get_bits() & self.get_u16(new_rock_top as usize, new_rock_left as usize)) == 0
        }
    }

    fn place_line(&mut self, row: u32, bits: u8, rock_left: u32) {
        let row_bits = match rock_left {
            6 => bits >> 3,
            5 => bits >> 2,
            4 => bits >> 1,
            3 => bits,
            2 => bits << 1,
            1 => bits << 2,
            0 => bits << 3,
            _ => panic!("Huh??"),
        };
        self.lines[row as usize] |= row_bits;
    }

    fn place_rock(&mut self, shape: Shape, rock_left: u32, rock_top: u32) {
        //println!("Placing {:?} {} {}", shape, rock_top, rock_left);
        // OR in with field, add new lines if necessary
        while rock_top >= self.lines.len() as u32 {
            self.lines.push(0b00);
        }
        match shape.get_max_height() {
            4 => {
                self.place_line(rock_top - 3, shape.get_bits_row(4), rock_left);
                self.place_line(rock_top - 2, shape.get_bits_row(3), rock_left);
                self.place_line(rock_top - 1, shape.get_bits_row(2), rock_left);
                self.place_line(rock_top, shape.get_bits_row(1), rock_left);
            }
            3 => {
                self.place_line(rock_top - 2, shape.get_bits_row(3), rock_left);
                self.place_line(rock_top - 1, shape.get_bits_row(2), rock_left);
                self.place_line(rock_top, shape.get_bits_row(1), rock_left);
            }
            2 => {
                self.place_line(rock_top - 1, shape.get_bits_row(2), rock_left);
                self.place_line(rock_top, shape.get_bits_row(1), rock_left);
            }
            1 => {
                self.place_line(rock_top, shape.get_bits_row(1), rock_left);
            }
            _ => panic!("Shouldn't get here!"),
        }
    }

    fn get_skyline(&self) -> u64 {
        let mut last_lines = [0_u8; 8];
        let len = self.lines.len();

        for i in 0..8 {
            last_lines[i] = self.lines[len - (1 + i)];
        }
        u64::from_ne_bytes(last_lines)
    }

    pub fn part_1(&mut self, num_rocks: usize) -> usize {
        let mut jet_space: usize = 0;
        let mut shape_num: usize = 0;
        let mut rock_num: usize = 0;
        let mut seen: HashMap<(u64, usize, usize), (usize, usize)> = HashMap::new();
        let mut cycle_height = 0;

        while rock_num < num_rocks {
            let shape = Self::get_shape(shape_num);
            shape_num = (shape_num + 1) % 5;
            let mut rock_left = 2;
            let mut rock_top = self.lines.len() + 2 + shape.get_max_height();

            loop {
                /*
                                println!(
                                    "New rock of type {:?} at {} {} - {} {}",
                                    shape, rock_top, rock_left, self.jets[jet_space as usize], jet_space
                                );
                */
                match self.jets[jet_space as usize] {
                    '<' => {
                        rock_left -= if self.can_move(shape, rock_top as i32, rock_left as i32 - 1)
                        {
                            1
                        } else {
                            0
                        }
                    }
                    '>' => {
                        rock_left += if self.can_move(shape, rock_top as i32, rock_left as i32 + 1)
                        {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Unexpected jet"),
                }
                jet_space = (jet_space + 1) % self.jets.len();
                if self.can_move(shape, rock_top as i32 - 1, rock_left as i32) {
                    rock_top -= 1;
                } else {
                    break;
                }
            }
            self.place_rock(shape, rock_left as u32, rock_top as u32);
            rock_num += 1;
            if self.lines.len() < 8 {
                continue;
            }
            // If we have seen same shape_num, same jet_index, and same top of cave
            let state = (self.get_skyline(), shape_num, jet_space);
            match seen.entry(state) {
                Entry::Occupied(e) => {
                    let (old_num, old_height) = e.get();
                    let num_rocks_in_cycle = rock_num - old_num;
                    let num_cycles = (num_rocks - rock_num) / num_rocks_in_cycle;
                    rock_num += num_rocks_in_cycle * num_cycles;
                    cycle_height += num_cycles * (self.lines.len() - old_height);
                    seen.clear();
                }
                Entry::Vacant(e) => {
                    e.insert((rock_num, self.lines.len()));
                }
            }
            //self.display();
        }
        self.lines.len() + cycle_height
    }
    fn display(&self) {
        for line in self.lines.iter().rev() {
            println!("## {:07b}", line);
        }
        println!("## -------");
    }
}
