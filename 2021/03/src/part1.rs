use std::{
    error::Error,
};
use crate::utils;

struct Counter(u64, u64);

impl Counter {
    fn new() -> Counter { Counter(0, 0) }
}

struct Diagnostic (
    Counter,
    Counter,
    Counter,
    Counter,
    Counter,
    Counter,
    Counter,
    Counter,
    Counter,
);

impl Diagnostic {
    fn new() -> Diagnostic {
        Diagnostic{
            Counter::new(),
            Counter::new(),
            Counter::new(),
            Counter::new(),
            Counter::new(),
            Counter::new(),
            Counter::new(),
            Counter::new(),
            Counter::new(),
        }
    }
}



// fn most_common(values: &Vec<[char; 12]>) -> HashMap<u8, Summary> {
//     let mut counts: HashMap<u8, Summary> = HashMap::new();


//     counts
// }

pub fn main() -> Result<(), Box<dyn Error>> {
    let values = utils::read_input()?;

    Ok(())
}
