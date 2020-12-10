use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn get_row(rows: &str) -> i32 {
    assert_eq!(rows.len(), 7);
    let mut lower = 0;
    let mut upper = 127;
    for letter in rows.chars().collect::<Vec<char>>() {
        let current_mod = (upper - lower + 1) / 2;
        if letter == 'F' {
            upper -= current_mod;
        } else if letter == 'B' {
            lower += current_mod;
        } else {
            println!("invalid row {}", letter);
            return -1;
        }
    }
    assert_eq!(lower, upper);
    lower
}

fn get_col(cols: &str) -> i32 {
    assert_eq!(cols.len(), 3);
    let mut lower = 0;
    let mut upper = 7;
    for letter in cols.chars().collect::<Vec<char>>() {
        let current_mod = (upper - lower + 1) / 2;
        if letter == 'R' {
            lower += current_mod;
        } else if letter == 'L' {
            upper -= current_mod;
        }
    }
    assert_eq!(lower, upper);
    lower
}

fn get_id(rows: &str, cols: &str) -> i32 {
    let row = get_row(rows);
    let col = get_col(cols);
    row * 8 + col
}

fn find_seat(ids: &Vec<i32>) -> i32 {
    for i in 1..ids.len() {
        if ids[i] - ids[i - 1] == 2 {
            return (ids[i] + ids[i - 1]) / 2;
        }
    }
    -1
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut vec = Vec::new();
    let mut ids = Vec::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        vec.push(my_string);
    }

    println!("Test Case {}", get_id("FBFBBFF", "RLR"));

    let mut highest_id = 0;
    for line in &vec {
        let rows = &line[0..7];
        let columns = &line[7..];
        let id = get_id(rows, columns);
        highest_id = max(highest_id, id);
        ids.push(id);
    }
    println!("Highest ID {}", highest_id);
    ids.sort();
    println!("Missing seat {}", find_seat(&ids));
    Ok(())
}
