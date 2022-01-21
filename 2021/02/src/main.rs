mod utils;
mod part1;
mod part2;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    part1::main()?;
    part2::main();
    Ok(())
}
