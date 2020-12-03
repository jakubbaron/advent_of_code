use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut vec: Vec<Vec<char>> = Vec::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        let chars = my_string.chars().collect::<Vec<char>>();
        vec.push(chars);
    }
    let mut i = 0;
    let mut j = 0;
    let mut counter = 0;
    while i < vec.len() && j < vec[0].len() {
        if vec[i][j] == '#' {
            counter += 1;
            vec[i][j] = 'X';
        }
        i += 1;
        j += 3;
        if j >= vec[0].len() {
            j = j - vec[0].len();
        }
    }
    for line in vec.iter() {
        let p: String = line.into_iter().collect();
        println!("{}", p);
    }
    println!("Encountered {} trees", counter);
    Ok(())
}
