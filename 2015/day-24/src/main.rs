use itertools::Itertools;
use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 99_usize, 44),
        ("input.txt", 10723906903_usize, 74850409),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let packages: Vec<usize> = file_content
            .into_iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let total_sum = packages.iter().fold(0, |acc, val| acc + val);
        let mut i = 1;
        let mut res_1 = (usize::MAX, usize::MAX);
        let mut res_2 = (usize::MAX, usize::MAX);
        while i < packages.len() {
            for v in packages.iter().combinations(i) {
                if v.iter().fold(0, |acc, val| acc + **val) * 4 == total_sum {
                    if v.len() < res_2.0 {
                        res_2 = (v.len(), v.iter().fold(1, |acc, val| acc * **val));
                    } else if v.len() == res_2.0 {
                        let t = std::cmp::min(res_2.1, v.iter().fold(1, |acc, val| acc * **val));
                        res_2 = (v.len(), t);
                    }
                }
                if v.iter().fold(0, |acc, val| acc + **val) * 3 == total_sum {
                    if v.len() < res_1.0 {
                        res_1 = (v.len(), v.iter().fold(1, |acc, val| acc * **val));
                    } else if v.len() == res_1.0 {
                        let t = std::cmp::min(res_1.1, v.iter().fold(1, |acc, val| acc * **val));
                        res_1 = (v.len(), t);
                    }
                    break;
                }
            }
            i += 1;
        }
        println!("Res 1: {:?}", res_1);
        assert_eq!(res_1.1, result_1);
        println!("Res 2: {:?}", res_2);
        assert_eq!(res_2.1, result_2);
    }
    Ok(())
}
