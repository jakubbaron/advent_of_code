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
            assert_eq!(1018336, my_int * (2020 - my_int));
            println!("{}", my_int * (2020 - my_int));
            break;
        }
        numbers.insert(2020 - my_int);
    }
    let mut vec = vec;
    vec.sort();
    let target_sum = 2020;
    for (i, el) in vec.iter().enumerate() {
        let mut right = vec.len() - 1;
        let mut left = i + 1;
        while left < right {
            let right_el = vec[right];
            let left_el = vec[left];
            let summed = el + left_el + right_el;
            if summed == target_sum {
                assert_eq!(288756720, right_el * left_el * el);
                println!("{}", right_el * left_el * el);
                break;
            } else if summed < target_sum {
                left += 1;
            } else if summed > target_sum {
                right -= 1;
            } else {
                println!("NOPE");
                break;
            }
        }
    }

    Ok(())
}
