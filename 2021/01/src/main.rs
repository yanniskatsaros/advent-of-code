use std::env::{current_dir};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::{Path};

fn read_input() -> Vec<i32> {
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

    lines
}

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

fn part1(measurements: &Vec<i32>) -> i32 {
    count_increasing(&measurements)
}

fn part2(measurements: &Vec<i32>) -> i32 {
    // we need to post-process the measurements using a "sliding window" of
    // three-measurements, summed together
    let n = measurements.len();
    let mut lines: Vec<i32> = vec![];

    for i in 2..n {
        let j = i-1;
        let k = i-2;

        let m1 = &measurements[i];
        let m2 = &measurements[j];
        let m3 = &measurements[k];
        lines.push(m1 + m2 + m3);
    }
    
    count_increasing(&lines)
}

fn main() {
    let measurements = read_input();

    let count = part1(&measurements);
    println!("Increasing depths (part 1): {}", count);

    let count = part2(&measurements);
    println!("Increasing depths (part 2): {}", count);
}
