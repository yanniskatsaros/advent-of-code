use std::env;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum Command {
    Noop,
    Addx(i32),
}

#[derive(Debug, Clone)]
struct Cpu {
    x: i32,
    cycle: u32,
}

#[derive(Debug)]
struct Signal {
    strength: i32,
}

impl Cpu {
    fn new() -> Self {
        Self { x: 1, cycle: 0 }
    }

    fn signal_strength(&self) -> i32 {
        self.x * (self.cycle as i32)
    }

    fn signal(&self) -> Option<Signal> {
        match self.cycle {
            20 | 60 | 100 | 140 | 180 | 220 => Some(Signal {
                strength: self.signal_strength(),
            }),
            _ => None,
        }
    }

    fn instruction(&mut self, cmd: &Command) -> Option<Signal> {
        use crate::Command::*;

        match cmd {
            Noop => {
                self.cycle += 1;
                self.signal()
            }
            Addx(i) => {
                self.cycle += 1;
                let sig1 = self.signal();

                self.cycle += 1;
                let sig2 = self.signal();

                self.x += i;

                sig1.or(sig2)
            }
        }
    }
}

impl Command {
    fn from(s: &str) -> Option<Self> {
        match s.split_once(" ") {
            None => match s {
                "noop" => Some(Self::Noop),
                _ => None,
            },
            Some(("addx", i)) => match i.parse::<i32>() {
                Ok(i) => Some(Self::Addx(i)),
                _ => None,
            },
            _ => None,
        }
    }
}

fn part1(input: &String) -> Result<(), Box<dyn Error>> {
    let mut cpu = Cpu::new();
    let mut signals = vec![];

    let cmds = input
        .split("\n")
        .into_iter()
        .flat_map(Command::from)
        .collect::<Vec<_>>();

    for cmd in cmds.iter() {
        if let Some(i) = cpu.instruction(cmd) {
            signals.push(i);
        }
    }

    let total: i32 = signals.into_iter().map(|s| s.strength).sum();
    println!("Part I: {total}");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read_to_string(path)?.trim().to_string();

    part1(&input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Command::*;

    #[test]
    fn test_parse_addx_pos() {
        let inp = Command::from("addx 1");
        let exp = Some(Addx(1));
        assert_eq!(exp, inp);
    }

    #[test]
    fn test_parse_addx_neg() {
        let inp = Command::from("addx -15");
        let exp = Some(Addx(-15));
        assert_eq!(exp, inp);
    }

    #[test]
    fn test_parse_noop() {
        let inp = Command::from("noop");
        let exp = Some(Noop);
        assert_eq!(exp, inp);
    }
}
