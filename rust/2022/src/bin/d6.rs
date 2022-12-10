use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

fn part1(s: &str) {
    let chars = s.chars().collect::<Vec<_>>();
    let indexes: Vec<_> = chars
        .windows(4)
        .enumerate()
        .filter_map(|(i, w)| {
            let set: HashSet<_, RandomState> = HashSet::from_iter(w);
            // add 4 since enumerating begins counting the first window at 0
            let j = i + 4;

            if set.len() == 4 {
                Some(j)
            } else {
                None
            }
        })
        // we only care about the first marker occurrence; discard the rest
        .take(1)
        .collect();

    let i = indexes[0];
    println!("Part I: {i}");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read_to_string(path)?.trim().to_string();

    part1(&input);

    Ok(())
}
