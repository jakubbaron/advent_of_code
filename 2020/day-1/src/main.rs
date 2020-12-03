use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut vec = Vec::new();
    let mut numbers = HashSet::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        let my_int = my_string.parse::<i32>().unwrap();
        vec.push(my_int);
    }

    for my_int in &vec {
        if numbers.contains(my_int) {
            println!("{}", my_int * (2020 - my_int));
            break;
        }
        numbers.insert(2020 - my_int);
    }

    for int1 in &vec {
        for int2 in &vec {
            for int3 in &vec {
                if int1 + int2 + int3 == 2020 {
                    println!("{}", int1 * int2 * int3);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
