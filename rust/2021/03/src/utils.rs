use std::{
    convert::TryInto,
};
use std::env::current_dir;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

// fn parse_line<const N: usize>(line: &str) -> [char; N] {
//     let chars = line.chars().collect::<Vec<char>>();

//     match chars.len() {
//         N => {

//         },
//         x => panic!("Expected exactly {} items, found: {}", N, x)
//     }

//     chars
// }

// fn parse_line(line: &str) -> [char; 12] {
//     let chars = line.chars().collect::<Vec<char>>();

//     chars.try_into()
//         .unwrap_or_else(|v: Vec<char>| {
//             return Err(format!("Expected exactly {} elements, found {}", 12, v))
//         })
// }

fn parse_digit(c: char) -> Option<u8> {
    c.to_digit(10).map(|d| d as u8)
}

fn parse_line(line: &str) -> Result<[u8; 12], String> {
    let nums: [u8; 12] = line.chars()
        .map(parse_digit)
        .collect::<Option<Vec<_>>>()
        .ok_or("failed to parse base 10 digit")?
        .try_into()
        .map_err(|_| "incorrect length, expected exactly 12 elements")?;
    
    Ok(nums)
}

pub fn read_input() -> Result<Vec<[u8; 12]>, String> {
    let pathbuf = current_dir()
        .unwrap()
        .join(Path::new("data/input.txt"));
    println!("Reading input data from: {}", pathbuf.display());

    let file = File::open(pathbuf).unwrap();
    let reader = BufReader::new(&file);
    let lines = reader.lines()
        .map(|l| parse_line(&l.unwrap())?)
        .collect::<Vec<_>>();
    
    Ok(lines)
}
