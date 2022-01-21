use std::{
    fmt,
    error::Error,
};

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

#[derive(Copy, Clone, Debug)]
pub enum Command {
    Forward,
    Down,
    Up,
}

#[derive(Copy, Clone, Debug)]
pub struct CommandLine(pub Command, pub i32);

impl CommandLine {
    pub fn parse(s: &str) -> Result<CommandLine, ParseError> {
        let mut iter = s.split_whitespace();
        let cmd = iter.next().ok_or(ParseError::MissingCommand)?;

        let value = iter.next().ok_or(ParseError::MissingValue)?;
        let value: i32 = value.parse()
            .map_err(|_| ParseError::InvalidValue(value.to_string()))?;
        
        // if there is any extra, this is an invalid command line
        if let Some(extra) = iter.next() {
            return Err(ParseError::InvalidExtraArguments(extra.to_string()));
        }

        // finally "tokenize" the command
        match cmd {
            "forward" => Ok(CommandLine(Command::Forward, value)),
            "down" => Ok(CommandLine(Command::Down, value)),
            "up" => Ok(CommandLine(Command::Up, value)),
            _ => Err(ParseError::InvalidCommand(cmd.to_string())),
        }

    }
}
