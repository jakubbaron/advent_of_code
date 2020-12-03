use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut numbers = HashSet::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        let my_int = my_string.parse::<i32>().unwrap();
        if numbers.contains(&my_int) {
            println!("{}", my_int * (2020 - my_int));
            return Ok(());
        }
        numbers.insert(2020 - my_int);
    }

    Ok(())
}
