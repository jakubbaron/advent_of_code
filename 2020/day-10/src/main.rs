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

    let mut ans: Vec<i64> = vec.to_vec().into_iter().map(|x| x as i64).collect();
    ans[0] = 1;
    for i in 2..vec.len() {
        let helper = match i >= 4 {
            true => vec[i-4],
            false => vec[i],
        };

        if vec[i] - helper == 4 {
            ans[i] = ans[i-4] * 7;
        } else if vec[i] - vec[i-2] == 2 {
            ans[i] = ans[i-1] * 2;
        } else {
            ans[i] = ans[i-1];
        }
    }
    println!("{:?}", vec);
    println!("{:?}", ans);
    println!("{}", ans.last().unwrap());


    Ok(())
}
