use std::{cell::RefCell, rc::Rc};
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use day11::Monkey;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let lines = lines_from_file(filename).expect("Could not load");

    let mut starting_line = 0;
    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = Vec::new();

    while starting_line < lines.len() {
        monkeys.push(Rc::new(RefCell::new(Monkey::new(
            &lines[starting_line..starting_line + 6],
        ))));
        starting_line += 7;
    }
    for round in 1..=20 {
        for monkey in &monkeys {
            let monkey: &mut Monkey = &mut monkey.borrow_mut();
            let mut done = false;
            while !done {
                match monkey.toss() {
                    None => done = true,
                    Some((dest, item)) => {
                        let mut dest_monkey = monkeys[dest].borrow_mut();
                        dest_monkey.add_item(item)
                    }
                }
            }
        }
        println!("After round {}", round);
        for monkey in &monkeys {
            let monkey = monkey.borrow();
            println!("Monkey {} {:?}", monkey.num_inspected, monkey.items);
        }
    }
    //println!("{:?}", monkeys);
}
