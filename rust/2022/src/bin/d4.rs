use std::env;
use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn parse(s: &str) -> Self {
        let parts: Vec<&str> = s.split('-').collect();
        match parts[..] {
            [a, b] => {
                let a = a.to_string().parse::<i32>().unwrap();
                let b = b.to_string().parse::<i32>().unwrap();
                Range { min: a, max: b }
            }
            _ => panic!("Error parsing unknown range format"),
        }
    }

    fn parse_pair(s: &str) -> (Self, Self) {
        let parts: Vec<&str> = s.split(',').collect();
        match parts[..] {
            [a, b] => {
                let a = Self::parse(a);
                let b = Self::parse(b);
                (a, b)
            }
            _ => panic!("Error parsing unknown range pair format"),
        }
    }

    fn contains(&self, other: &Self) -> bool {
        (self.min <= other.min) & (self.max >= other.max)
    }

    fn overlaps(&self, other: &Self) -> bool {
        ((self.min <= other.min) & (self.max >= other.min))
            | ((self.min <= other.max) & (self.max >= other.max))
            | other.contains(&self)
    }
}

fn either_contains(a: &Range, b: &Range) -> bool {
    a.contains(b) | b.contains(a)
}

fn part1(input: &String) -> Result<(), Box<dyn Error>> {
    let total: i32 = input
        .split('\n')
        .into_iter()
        .map(|s| {
            let (a, b) = Range::parse_pair(s);
            match either_contains(&a, &b) {
                true => 1,
                false => 0,
            }
        })
        .sum();

    println!("Part I: {total}");
    Ok(())
}

fn part2(input: &String) -> Result<(), Box<dyn Error>> {
    let total: i32 = input
        .split('\n')
        .into_iter()
        .map(|s| {
            let (a, b) = Range::parse_pair(s);
            match a.overlaps(&b) {
                true => 1,
                false => 0,
            }
        })
        .sum();

    println!("Part II: {total}");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read_to_string(path)?.trim().to_string();

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_parse() {
        let r = "1-2";
        let range = Range { min: 1, max: 2 };

        assert_eq!(Range::parse(r), range)
    }

    #[test]
    fn test_range_parse_pair() {
        let pair = "1-2,3-4";
        let a = Range { min: 1, max: 2 };
        let b = Range { min: 3, max: 4 };

        assert_eq!(Range::parse_pair(pair), (a, b))
    }

    #[test]
    fn test_contains() {
        //    0 1 2 3 4 5
        // a: - * * * * -
        // b: - - * * - -
        let a = Range { min: 1, max: 4 };
        let b = Range { min: 2, max: 3 };

        assert!(a.contains(&b));
    }

    #[test]
    fn test_no_overlaps() {
        //    0 1 2 3 4 5 6 7 8
        // a: - - * * * - - - -
        // b: - - - - - - * * *
        let a = Range { min: 2, max: 4 };
        let b = Range { min: 6, max: 8 };
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn test_overlaps_partial() {
        //    0 1 2 3 4 5
        // a: - * * * - -
        // b: - - * * * -
        let a = Range { min: 1, max: 3 };
        let b = Range { min: 2, max: 4 };
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_overlaps_boundary() {
        //    4 5 6 7 8 9
        // a: - * * * - -
        // b: - - - * * *
        let a = Range { min: 5, max: 7 };
        let b = Range { min: 7, max: 9 };
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_overlaps_enclosed() {
        //    0 1 2 3 4 5 6 7 8
        // a: - - * * * * * * *
        // b: - - - * * * * * -
        let a = Range { min: 2, max: 8 };
        let b = Range { min: 3, max: 7 };
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }
}
