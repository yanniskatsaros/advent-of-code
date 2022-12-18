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
    image: String,
}

#[derive(Debug)]
struct Signal {
    strength: i32,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            cycle: 0,
            image: String::new(),
        }
    }

    fn draw(&mut self) {
        let i = self.x - 1;
        let j = self.x + 1;
        let crt = ((self.cycle as i32) - 1) % 40;

        if (crt >= i) && (crt <= j) {
            self.image.push_str("#");
        } else {
            self.image.push_str(".");
        }

        match self.cycle {
            40 | 80 | 120 | 160 | 200 | 240 => {
                self.image.push_str("\n");
            }
            _ => (),
        }
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
                self.draw();
                self.signal()
            }
            Addx(i) => {
                self.cycle += 1;
                self.draw();
                let sig1 = self.signal();

                self.cycle += 1;
                self.draw();
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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut cpu = Cpu::new();
    let mut signals = vec![];

    let cmds = read_to_string(path)?
        .trim()
        .to_string()
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
    println!("Part II:\n\n{}", cpu.image);

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
