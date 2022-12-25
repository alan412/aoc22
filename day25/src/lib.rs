#[derive(Debug)]
pub struct Puzzle {
    amount_fuel: Vec<Snafu>,
}

#[derive(Debug)]
pub struct Snafu {
    val: i64,
}

impl Snafu {
    fn new(line: &str) -> Self {
        let mut val = 0;
        let mut place = 0;
        let mut str = line.to_string();
        loop {
            match str.pop() {
                None => return Self { val },
                Some(ch) => {
                    val += match ch {
                        '2' => 2 * 5_i64.pow(place),
                        '1' => 1 * 5_i64.pow(place),
                        '0' => 0 * 5_i64.pow(place),
                        '-' => -1 * 5_i64.pow(place),
                        '=' => -2 * 5_i64.pow(place),
                        _ => unreachable!(),
                    };
                    place += 1;
                }
            }
        }
    }
    fn from_val(val: i64) -> Self {
        Self { val }
    }
    fn get_string(&self) -> String {
        let mut number = self.val;
        let mut result = String::new();

        while number > 0 {
            let remainder = number % 5;
            result.insert(
                0,
                match remainder {
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    3 => '=',
                    4 => '-',
                    _ => unreachable!(),
                },
            );
            if remainder >= 3 {
                number += 5 - remainder
            }
            number /= 5;
        }
        return result;
    }
}
impl Puzzle {
    pub fn new() -> Self {
        Self {
            amount_fuel: Vec::new(),
        }
    }
    pub fn add_line(&mut self, line: &str) {
        self.amount_fuel.push(Snafu::new(line));
    }
    pub fn pt_1(&mut self) -> i64 {
        let mut total = 0;
        for fuel in self.amount_fuel.iter() {
            total += fuel.val;
        }
        let snafu = Snafu::from_val(total);

        println!("Total: {} {:?}", snafu.get_string(), snafu);
        0
    }
}
