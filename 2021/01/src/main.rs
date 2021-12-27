use std::env::{current_dir};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::{Path};

fn count_increasing(measurements: &Vec<i32>) -> i32 {
    let mut count = 0;
    let mut prev = &measurements[0];

    for m in measurements {
        if m > prev {
            count += 1;
        }
        prev = m;
    }
    count
}

fn part1() -> i32 {
    let pathbuf = current_dir()
        .unwrap()
        .join(Path::new("data/input.txt"));

    println!("Reading data from: {}", pathbuf.display());

    let fp  = File::open(pathbuf).unwrap();
    let file = BufReader::new(&fp);

    let mut lines: Vec<i32> = vec![];
    for line in file.lines() {
        lines.push(line.unwrap().parse().unwrap());
    }

    count_increasing(&lines)
}

fn main() {
    let count = part1();
    println!("Increasing depths (part 1): {}", count);
}
