use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn is_sum(slice: &[i64], number: i64) -> bool {
    let mut diffs: HashSet<i64> = HashSet::new();
    for el in slice.iter() {
        if diffs.contains(&el) {
            return true;
        }
        diffs.insert(number - el);
    }
    return false;
}

fn main() -> io::Result<()> {
    // let f = File::open("test.txt")?; let preamble_len = 5; let mut answer = 127;
    let f = File::open("input.txt")?;
    let preamble_len = 25;
    let mut answer = 466456641;
    let f = BufReader::new(f);
    let mut vec = Vec::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        vec.push(my_string);
    }
    let vec: Vec<i64> = vec.into_iter().map(|x| x.parse::<i64>().unwrap()).collect();

    for i in preamble_len..vec.len() {
        if !is_sum(&vec[i - preamble_len..i], vec[i]) {
            answer = vec[i];
            println!("{}", answer);
            break;
        }
    }

    let mut has_answer = false;
    for i in 0..vec.len() {
        for j in (i + 1)..vec.len() {
            let slice: &[i64] = &vec[i..j];
            let sum: i64 = slice.iter().sum();
            if sum == answer {
                let min_val = slice.iter().min().unwrap();
                let max_val = slice.iter().max().unwrap();
                println!("{} {} {}", min_val, max_val, min_val + max_val);
                has_answer = true;
                break;
            } else if sum > answer {
                break;
            }
        }
        if has_answer {
            break;
        }
    }

    Ok(())
}
