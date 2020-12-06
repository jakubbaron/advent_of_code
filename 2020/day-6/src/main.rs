use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    // let f = File::open("test.txt")?;
    let f = BufReader::new(f);
    let mut vec = Vec::new();
    let mut letters: HashSet<char> = HashSet::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        vec.push(my_string);
    }

    let mut counter = 0;
    for line in &vec {
        if line.is_empty() {
            counter += letters.len();
            letters.clear();
        }
        let temp: HashSet<char> = line.chars().collect();
        letters.extend(&temp);
    }

    if !letters.is_empty() {
        counter += letters.len();
        letters.clear();
    }
    println!("{} answers", counter);

    let mut char_count: HashMap<char, i32> = HashMap::new();
    let mut group_len = 0;
    let mut counter_2 = 0;
    for line in &vec {
        if line.is_empty() {
            // println!("{:?} {}", char_count, group_len);
            for (k, v) in char_count.iter() {
                if *v == group_len {
                    counter_2 += 1;
                }
            }
            char_count.clear();
            group_len = 0;
            continue;
        }
        group_len += 1;
        for c in line.chars().collect::<Vec<char>>() {
            match char_count.get_mut(&c) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    char_count.insert(c, 1);
                }
            }
        }
    }
    for (k, v) in char_count.iter() {
        if *v == group_len {
            counter_2 += 1;
        }
    }

    println!("{} common answers", counter_2);
    Ok(())
}
