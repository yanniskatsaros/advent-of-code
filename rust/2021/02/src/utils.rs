use std::env::current_dir;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

pub fn read_input() -> Vec<String> {
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
