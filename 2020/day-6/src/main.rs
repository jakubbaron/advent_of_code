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
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut tmp = Vec::new();
    for line in vec {
        if line.is_empty() {
            groups.push(tmp.to_vec());
            tmp.clear();
            continue;
        }
        tmp.push(line);
    }
    groups.push(tmp.to_vec());

    let groups = groups;

    let mut counter = 0;
    for group in groups.iter() {
        for line in group.iter() {
            if line.is_empty() {
            }
            let temp: HashSet<char> = line.chars().collect();
            letters.extend(&temp);
        }
        counter += letters.len();
        letters.clear();
    }

    println!("{} answers", counter);

    let mut char_count: HashMap<char, usize> = HashMap::new();
    let mut counter_2 = 0;
    for group in groups.iter() {
        for line in group.iter() {
            for c in line.chars().collect::<Vec<char>>() {
                *char_count.entry(c).or_insert(0) += 1
            }
        }
        for (k, v) in char_count.iter() {
            if *v == group.len() {
                counter_2 += 1;
            }
        }
        char_count.clear();
        continue;
    }

    println!("{} common answers", counter_2);
    Ok(())
}
