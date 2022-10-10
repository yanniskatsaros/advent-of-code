use crate::parse::{
    Command,
    CommandLine,
    ParseError,
};
use crate::utils;

#[derive(Debug)]
struct Submarine {
    position: i32,
    depth: i32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine { position: 0, depth: 0 }
    }

    fn run(&mut self, cmd: &CommandLine) {
        match cmd {
            CommandLine(Command::Forward, value) => self.position += value,
            CommandLine(Command::Down, value) => self.depth += value,
            CommandLine(Command::Up, value) => self.depth -= value,
        }
    }
}


pub fn main() -> Result<(), ParseError> {
    println!("Day 2 - Part I");
    let mut sub = Submarine::new();
    println!("{:?}", sub);

    let instructions = utils::read_input().iter()
        .map(|s| CommandLine::parse(s))
        .collect::<Result<Vec<_>, _>>()?;
    for i in &instructions {
        sub.run(i);
    }
    println!("{:?}", sub);
    println!("Answer = {}", sub.depth * sub.position);

    Ok(())
}
