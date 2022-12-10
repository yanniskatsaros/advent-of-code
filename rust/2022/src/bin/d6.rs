use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

fn decoder(s: &str, n: i32) -> i32 {
    let n = n as usize;
    let chars = s.chars().collect::<Vec<_>>();
    let indexes: Vec<_> = chars
        .windows(n as usize)
        .enumerate()
        .filter_map(|(i, w)| {
            let set: HashSet<_, RandomState> = HashSet::from_iter(w);
            // add n since enumerating begins counting the first window at 0
            let j = i + n;

            if set.len() == n {
                Some(j)
            } else {
                None
            }
        })
        // we only care about the first marker occurrence; discard the rest
        .take(1)
        .collect();

    indexes[0] as i32
}

fn part1(s: &str) {
    let i = decoder(s, 4);
    println! {"Part I: {i}"};
}

fn part2(s: &str) {
    let i = decoder(s, 14);
    println! {"Part II: {i}"};
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read_to_string(path)?.trim().to_string();

    part1(&input);
    part2(&input);

    Ok(())
}
