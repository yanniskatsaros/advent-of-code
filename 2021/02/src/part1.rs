use std::env::current_dir;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

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


fn read_input() -> Vec<String> {
    let pathbuf = current_dir()
        .unwrap()
        .join(Path::new("data/input.txt"));
    println!("Reading input data from: {}", pathbuf.display());

    let file = File::open(pathbuf).unwrap();
    let reader = BufReader::new(&file);
    let mut lines: Vec<String> = vec![];

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}


pub fn main() {
    println!("Day 2 - Part I");
    let mut sub = Submarine { position: 0, depth: 0 };
    println!("{:?}", sub);

    let instructions = read_input();
    for line in instructions {
        sub.parse_instruction(&line);
    }
    println!("{:?}", sub);
    println!("Answer = {}", sub.depth * sub.position);
}
