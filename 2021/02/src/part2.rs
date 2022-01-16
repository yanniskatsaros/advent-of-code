use crate::utils;

#[derive(Debug)]
struct Submarine {
    position: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    fn parse_instruction(&mut self, s: &String) {
        let mut iter = s.split_whitespace();
        let cmd = iter.next().expect("Missing command from instruction.");
        let value: i32 = iter.next()
            .expect("Missing value from instruction.")
            .parse()
            .expect("Value must be an integer type.");
        
        // if there are not exactly two instructions, panic
        match iter.next() {
            None => (),
            Some(extra) => panic!("Too many instructions found: {}", extra)
        }

        match cmd {
            "forward" => {
                self.position += value;
                self.depth += self.aim * value;
            },
            "down" => self.aim += value,
            "up" => self.aim -= value,
            other => panic!("Invalid command found: {}", other),
        }
    }

    fn new() -> Submarine {
        Submarine {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }
}

pub fn main() {
    println!("Day 2 - Part II");
    let mut sub = Submarine::new();

    let instructions = utils::read_input();
    for line in instructions {
        sub.parse_instruction(&line);
    }
    println!("{:?}", sub);
    println!("Answer = {}", sub.depth * sub.position);
}
