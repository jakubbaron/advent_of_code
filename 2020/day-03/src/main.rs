use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

// fn print_slope(vec: &Vec<Vec<char>>) {
//     for line in vec.iter() {
//         let p: String = line.into_iter().collect();
//         println!("{}", p);
//     }
// }

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut vec: Vec<Vec<char>> = Vec::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        let chars = my_string.chars().collect::<Vec<char>>();
        vec.push(chars);
    }
    let tuples = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut end: u64 = 1;
    for (right, down) in tuples.iter() {
        let mut i = 0;
        let mut j = 0;
        let mut counter = 0;
        while i < vec.len() && j < vec[0].len() {
            if vec[i][j] == '#' {
                counter += 1;
            }
            i += down;
            j += right;
            if j >= vec[0].len() {
                j = j - vec[0].len();
            }
        }
        println!(
            "Encountered {} trees right={} down={}",
            counter, right, down
        );
        end *= counter;
    }
    println!("Multiplied {} trees", end);
    Ok(())
}
