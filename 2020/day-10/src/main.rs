use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("test.txt")?;
    let f = File::open("test2.txt")?;
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut vec = Vec::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        vec.push(my_string);
    }
    let mut vec: Vec<i32> = vec.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    vec.sort();
    let vec = vec;
    let mut count: HashMap<i32, i32> = HashMap::new();
    // println!("{:?}", vec);

    *count.entry(&vec[0] - &0).or_insert(0) += 1;
    for i in 1..vec.len() {
        let diff = &vec[i] - &vec[i-1];
        *count.entry(diff).or_insert(0) += 1;
    }
    *count.entry(3).or_insert(0) += 1;
    for (k, v) in count.iter() {
        println!("{} {}", k, v);
    }
    println!("{}", count.get(&1).unwrap() * count.get(&3).unwrap());
    Ok(())
}
