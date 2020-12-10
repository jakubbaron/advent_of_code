// use std::collections::HashMap;
// use std::collections::HashSet;
use std::io::{self};

fn main() -> io::Result<()> {
    let f = "test.txt";
    // let f = "input.txt";

    let vec: Vec<i32> = std::fs::read_to_string(f)?
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for v in vec.iter() {
        println!("{}", v);
    }
    Ok(())
}
