// use std::collections::HashMap;
// use std::collections::HashSet;
use std::io::{self};

fn main() -> io::Result<()> {
    let f = "test.txt";
    // let f = "input.txt";

    let vec: Vec<String> = std::fs::read_to_string(f)?
        .lines()
        .map(|x| x.to_string())
        .collect();

    let vec: Vec<Vec<char>> = vec.into_iter().map(|x| x.replace("L", "#").chars().collect()).collect();
    for v in vec.iter() {
        println!("{:?}", v);
    }
    let mut after = vec.to_vec();
    let mut before = vec.to_vec();

    loop {
        let mut changed = false;
        for (i, row) in before.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                let ch = *ch;
                if ch == '.' {
                    continue;
                }
                let mut adj = 0;

                let occ = '#';
                // above
                if i > 0 {
                    if j > 0 && before[i-1][j-1] == occ {
                        adj += 1;
                    }
                    if before[i-1][j] == occ {
                        adj += 1;
                    }
                    if j < row.len() - 1 && before[i-1][j+1] == occ {
                        adj += 1;
                    }
                }
                // same row
                if j > 0 && before [i][j-1] == occ {
                    adj += 1;
                }
                if j < row.len() - 1 && before[i][j+1] == occ {
                    adj += 1;
                }
                // below
                if i < before.len() - 1 {
                    if j > 0 && before[i+1][j-1] == occ {
                        adj += 1;
                    }
                    if before[i+1][j] == occ {
                        adj += 1;
                    }
                    if j < row.len() - 1 && before[i+1][j+1] == occ {
                        adj += 1;
                    }
                }

                if ch == '#' && adj >= 4 {
                    after[i][j] = 'L';
                    changed = true;
                }
                if ch == 'L' && adj == 0 {
                    after[i][j] = '#';
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
        println!("New round");
        for v in after.iter() {
            println!("{:?}", v);
        }
        before = after.to_vec();
    }

    let mut sum = 0;
    for v in after.iter() {
        for c in v.iter() {
            if *c == '#' {
                sum += 1;
            }
        }
    }
    println!("Occupied seats: {}", sum);
    Ok(())
}
