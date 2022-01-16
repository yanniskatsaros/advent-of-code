use crate::utils;

#[derive(Debug)]
struct Submarine {
    position: i32,
    depth: i32,
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
            "forward" => self.position += value,
            "down" => self.depth += value,
            "up" => self.depth -= value,
            other => panic!("Invalid command found: {}", other),
        }
    }
}


pub fn main() {
    println!("Day 2 - Part I");
    let mut sub = Submarine { position: 0, depth: 0 };
    println!("{:?}", sub);

    let instructions = utils::read_input();
    for line in instructions {
        sub.parse_instruction(&line);
    }
    println!("{:?}", sub);
    println!("Answer = {}", sub.depth * sub.position);
}
