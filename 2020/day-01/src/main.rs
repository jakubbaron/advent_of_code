use std::collections::HashSet;
use std::io::{self};

fn main() -> io::Result<()> {
    let f = "input.txt";
    let vec: Vec<i32> = std::fs::read_to_string(f)?
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut numbers = HashSet::new();
    for my_int in &vec {
        if numbers.contains(my_int) {
            println!("{}", my_int * (2020 - my_int));
            break;
        }
        numbers.insert(2020 - my_int);
    }

    for int1 in &vec {
        for int2 in &vec {
            for int3 in &vec {
                if int1 + int2 + int3 == 2020 {
                    println!("{}", int1 * int2 * int3);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
