use std::collections::HashMap;
use std::collections::HashSet;
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
    vec.insert(0, 0);
    vec.insert(vec.len(), vec.last().unwrap() + 3);
    let vec = vec;
    let mut count: HashMap<i32, i32> = HashMap::new();
    for i in 1..vec.len() {
        let diff = &vec[i] - &vec[i-1];
        *count.entry(diff).or_insert(0) += 1;
    }
    // for (k, v) in count.iter() {
    //     println!("{} {}", k, v);
    // }
    println!("{}", count.get(&1).unwrap() * count.get(&3).unwrap());

    // let available: HashSet<i32> = vec.to_vec().into_iter().collect();

    let mut pow2 = 0;
    let mut pow7 = 0;

    for i in 1..vec.len() - 1 {
        let negative;
        if i >= 3 {
            negative = vec[i-3];
        } else {
            negative = -9999;
        }
        if vec[i+1] - negative == 4 {
            pow7 += 1;
            pow2 -= 2;
        } else if vec[i+1] - vec[i-1] == 2 {
            pow2 += 1;
        }
    }
    println!("{}", 2_i64.pow(pow2) * 7_i64.pow(pow7));
    Ok(())
}
