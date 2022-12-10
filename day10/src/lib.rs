use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    NOP,
    ADDX(i32),
}

#[derive(Debug)]
pub struct Cpu {
    program: Vec<Instruction>,
    x_at: HashMap<u32, i32>,
    highest_cycle: u32,
}

impl Cpu {
    pub fn new() -> Self {
        let mut x_at = HashMap::new();
        x_at.insert(0, 1);
        Self {
            program: Vec::new(),
            x_at: x_at,
            highest_cycle: 0,
        }
    }
    pub fn add_line(&mut self, line: String) {
        let v: Vec<String> = line.split_whitespace().map(str::to_string).collect();
        if v[0] == "noop" {
            self.program.push(Instruction::NOP);
        } else {
            let op1 = v[1].parse::<i32>().unwrap();
            self.program.push(Instruction::ADDX(op1));
        }
    }
    pub fn execute(&mut self) {
        let mut cycle: u32 = 0;
        let mut x_reg: i32 = 1;

        for inst in &self.program {
            match inst {
                Instruction::NOP => cycle += 1,
                Instruction::ADDX(op1) => {
                    cycle += 2;
                    x_reg += op1;
                    self.x_at.insert(cycle, x_reg);
                }
            }
        }
        self.highest_cycle = cycle;
    }
    pub fn get_x_at(&self, cycle: i32) -> i32 {
        let mut check_cycle: u32 = cycle.try_into().unwrap();
        check_cycle -= 1; // check at beginning, not end

        if check_cycle > self.highest_cycle {
            check_cycle = self.highest_cycle;
        }

        while check_cycle > 1 {
            match self.x_at.get(&check_cycle) {
                Option::None => check_cycle -= 1,
                Option::Some(val) => {
                    // println!("Returning {} at {}", val, check_cycle);
                    return *val;
                }
            }
        }
        return 1;
    }
}
