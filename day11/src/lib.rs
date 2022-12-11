#[derive(Debug)]
enum Operation {
    TIMES(i32),
    ADD(i32),
    SQUARE,
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<i32>,
    op: Operation,
    test: u32,
    pass_true: usize,
    pass_false: usize,
    pub num_inspected: u32,
}

impl Monkey {
    fn parse_items(&mut self, line: &String) {
        let tmp_string: String = line.replace(",", " ");
        let tokens: Vec<&str> = tmp_string.split_whitespace().collect();

        for index in 2..tokens.len() {
            self.items.push(tokens[index].parse().unwrap());
        }
    }
    fn parse_operation(&mut self, line: &String) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens[4] {
            "*" => {
                if tokens[5] == "old" {
                    self.op = Operation::SQUARE;
                } else {
                    self.op = Operation::TIMES(tokens[5].parse().unwrap());
                }
            }
            "+" => self.op = Operation::ADD(tokens[5].parse().unwrap()),
            _ => panic!("Unknown token!"),
        }
    }
    fn parse_test(&mut self, line: &String) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        self.test = tokens[tokens.len() - 1].parse().unwrap();
    }
    fn parse_pass_true(&mut self, line: &String) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        self.pass_true = tokens[tokens.len() - 1].parse().unwrap();
    }
    fn parse_pass_false(&mut self, line: &String) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        self.pass_false = tokens[tokens.len() - 1].parse().unwrap();
    }

    pub fn new(lines: &[String]) -> Self {
        let mut me = Self {
            items: Vec::new(),
            op: Operation::TIMES(4),
            test: 17,
            pass_true: 0,
            pass_false: 0,
            num_inspected: 0,
        };
        me.parse_items(&lines[1]);
        me.parse_operation(&lines[2]);
        me.parse_test(&lines[3]);
        me.parse_pass_true(&lines[4]);
        me.parse_pass_false(&lines[5]);
        me
    }
    pub fn add_item(&mut self, item: i32) {
        self.items.push(item);
    }
    pub fn toss(&mut self) -> Option<(usize, i32)> {
        if self.items.len() == 0 {
            None
        } else {
            self.num_inspected += 1;
            let mut item = self.items.pop().unwrap();
            //println!(" starting Item: {}", item);
            item = match self.op {
                Operation::TIMES(op2) => item * op2,
                Operation::ADD(op2) => item + op2,
                Operation::SQUARE => item * item,
            };
            //println!(" after op: {}", item);
            item = item / 3;
            //println!(" after div: {}", item);
            let dest = if (item % self.test as i32) == 0 {
                self.pass_true
            } else {
                self.pass_false
            };
            Some((dest, item))
        }
    }
}
