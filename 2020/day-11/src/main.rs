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

    let vec: Vec<Vec<char>> = vec
        .into_iter()
        .map(|x| x.replace("L", "#").chars().collect())
        .collect();
    for v in vec.iter() {
        println!("{:?}", v);
    }
    let directions = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let occ = '#';
    let free = 'L';
    let floor = '.';

    let mut after = vec.to_vec();
    let mut before = vec.to_vec();

    loop {
        let mut changed = false;
        for (i, row) in before.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                let ch = *ch;
                if ch == floor {
                    continue;
                }
                let mut adj = 0;

                for dir in directions.iter() {
                    let (horiz, vert) = dir;
                    let cj = j as i32 + horiz;
                    let ci = i as i32 + vert;
                    if ci < 0 || ci >= before.len() as i32 || cj < 0 || cj >= row.len() as i32 {
                        continue;
                    }
                    if before[ci as usize][cj as usize] == occ {
                        adj += 1;
                    }
                }

                if ch == occ && adj >= 4 {
                    after[i][j] = free;
                    changed = true;
                }
                if ch == free && adj == 0 {
                    after[i][j] = occ;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
        // println!("New round");
        // for v in after.iter() {
        //     println!("{:?}", v);
        // }
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

    let mut after = vec.to_vec();
    let mut before = vec.to_vec();

    loop {
        let mut changed = false;
        for (i, row) in before.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                let ch = *ch;
                if ch == floor {
                    continue;
                }
                let mut occ_count = 0;

                for dir in directions.iter() {
                    let (horiz, vert) = dir;
                    let mut cj = j as i32 + horiz;
                    let mut ci = i as i32 + vert;
                    while ci >= 0 && ci < before.len() as i32 && cj >= 0 && cj < row.len() as i32 {
                        if before[ci as usize][cj as usize] == free {
                            break;
                        }
                        if before[ci as usize][cj as usize] == occ {
                            occ_count += 1;
                            break;
                        }
                        cj = cj + horiz;
                        ci = ci + vert;
                    }
                }
                if ch == occ && occ_count >= 5 {
                    after[i][j] = free;
                    changed = true;
                }
                if ch == free && occ_count == 0 {
                    after[i][j] = occ;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
        // println!("New round");
        // for v in after.iter() {
        //     println!("{:?}", v);
        // }
        before = after.to_vec();
    }

    let mut sum = 0;
    for v in after.iter() {
        for c in v.iter() {
            if *c == occ {
                sum += 1;
            }
        }
    }
    println!("Occupied seats: {}", sum);
    Ok(())
}
