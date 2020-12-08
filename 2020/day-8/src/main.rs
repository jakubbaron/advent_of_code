use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};


fn main() -> io::Result<()> {
    let f = File::open("test.txt")?;
    // let f = File::open("test2.txt")?;
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut vec = Vec::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        vec.push(my_string);
    }

    let mut id:i32 = 0;
    let mut acc = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    loop {
        visited.insert(id.clone());
        let tmp: Vec<&str> = vec[id as usize].split(" ").collect();
        let instr: &str = tmp[0].trim();
        let number = tmp[1].parse::<i32>().unwrap();
        let mut new_id = -1;
        let mut new_acc = acc;
        if instr == "nop" {
            new_id = id + 1;
        } else if instr == "acc" {
            new_acc += number;
            new_id = id + 1;
        } else if instr == "jmp" {
            new_id = id + number;
        }

        if visited.contains(&new_id) {
            println!("Value in the accumulator: {}", acc);
            break;
        }
        id = new_id;
        acc = new_acc;
    }

    Ok(())
}
