use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let expected: HashSet<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let mut counter = 0;
    let mut received: HashSet<String> = HashSet::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        let field_values: Vec<&str> = my_string.split(" ").collect();
        for fv in field_values {
            let tmp: Vec<&str> = fv.split(":").collect();
            let field = String::from(tmp[0]);
            if !field.is_empty() {
                received.insert(field);
            }
        }
        if my_string.is_empty() {
            if received.is_superset(&expected) {
                counter += 1;
            }
            received.clear();
        }
    }

    if !received.is_empty() {
        if received.is_superset(&expected) {
            counter += 1;
        }
        received.clear();
    }

    println!("Valid passports {}", counter);

    Ok(())
}
