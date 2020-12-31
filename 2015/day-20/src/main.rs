use std::collections::HashMap;
use std::io::{self};

fn divisors_multiplied(n: usize) -> usize {
    let mut result = 0;
    for i in 1..((n as f64).sqrt() as usize + 1) {
        if n % i == 0 {
            result += i;
            if n / i != i {
                result += n / i;
            }
        }
    }
    result
}

fn presents_in_the_house(house: usize) -> usize {
    divisors_multiplied(house) * 10
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 8, 6), ("input.txt", 665280, 705600)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let line = &file_content[0];
        let number = line.parse::<usize>().unwrap();
        let mut house_no = 1;
        while presents_in_the_house(house_no) < number {
            house_no += 1;
        }
        println!("House {} received more than {} presents", house_no, number);
        assert_eq!(house_no, result_1);

        let mut houses: HashMap<usize, usize> = HashMap::new();
        let mut res_2 = usize::MAX;
        for elf in 1..number / 10 {
            for i in 1..=50 {
                if *houses
                    .entry(elf * i)
                    .and_modify(|x| *x += elf * 11)
                    .or_insert(elf * 11)
                    >= number
                {
                    let elfi = elf * i;
                    if elfi < res_2 {
                        res_2 = elfi;
                    }
                }
            }
        }
        assert_eq!(res_2, result_2);
    }

    Ok(())
}
