use std::{
    fmt,
    error::Error,
};
use crate::utils;

#[derive(Debug)]
pub enum ParseError {
    MissingCommand,
    MissingValue,
    InvalidCommand(String),
    InvalidValue(String),
    InvalidExtraArguments(String),
}

impl Error for ParseError { }

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Submarine {
    position: i32,
    depth: i32,
}

impl Submarine {
    fn parse_instruction(&mut self, s: &str) -> Result<(), ParseError> {
        let mut iter = s.split_whitespace();

        let cmd = iter.next().ok_or(ParseError::MissingCommand)?;
        // this is equivalent (syntactic sugar for) the below:
        // let cmd = match iter.next() {
        //     Some(cmd) => cmd,
        //     None => return Err(ParseError::MissingCommand)
        // };

        let value = iter.next().ok_or(ParseError::MissingValue)?;
        let value: i32 = value.parse()
            .map_err(|_| ParseError::InvalidValue(value.to_string()))?;

        // if there are not exactly two instructions, this is also invalid
        // if-let syntax is like a one-armed match statement
        if let Some(extra) = iter.next() {
            return Err(ParseError::InvalidExtraArguments(extra.to_string()))
        }

        match cmd {
            "forward" => self.position += value,
            "down" => self.depth += value,
            "up" => self.depth -= value,
            _ => return Err(ParseError::InvalidCommand(cmd.to_string()))
        }

        Ok(())
    }
}


pub fn main() -> Result<(), ParseError> {
    println!("Day 2 - Part I");
    let mut sub = Submarine { position: 0, depth: 0 };
    println!("{:?}", sub);

    let instructions = utils::read_input();
    for line in instructions {
        sub.parse_instruction(&line)?;
    }
    println!("{:?}", sub);
    println!("Answer = {}", sub.depth * sub.position);

    Ok(())
}
