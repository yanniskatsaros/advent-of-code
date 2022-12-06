use std::env;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = read_to_string(path)?.trim().to_string();

    dbg!(input);
    Ok(())
}
