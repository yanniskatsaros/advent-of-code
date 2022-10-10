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
    aim: i32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            position: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn run(&mut self, cmd: &CommandLine) {
        match cmd {
            CommandLine(Command::Forward, value) => {
                self.position += value;
                self.depth += self.aim * value;
            },
            CommandLine(Command::Down, value) => self.aim += value,
            CommandLine(Command::Up, value) => self.aim -= value,
        }
    }
}

pub fn main() -> Result<(), ParseError>{
    println!("Day 2 - Part II");
    let mut sub = Submarine::new();

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
