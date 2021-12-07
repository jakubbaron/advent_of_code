use std::cmp::min;
use std::collections::HashMap;
use std::io::{self};

fn parse_input(file_content: &Vec<String>) -> Vec<i32> {
    file_content[0]
        .split(",")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn part_1(file_content: &Vec<String>) -> i32 {
    let data = parse_input(&file_content);
    let mut helper: HashMap<i32, i32> = HashMap::new();
    let mut current_min = i32::MAX;
    for i in 0..data.len() {
        let curr_el = data[i];
        for j in 0..data.len() {
            if i == j {
                continue;
            }
            let other_el = data[j];
            let entry = helper.entry(curr_el).or_insert(0);
            *entry += (curr_el - other_el).abs();
        }
        current_min = min(current_min, *helper.values().min().unwrap());
    }
    current_min
}

fn part_2(file_content: &Vec<String>) -> i32 {
    let data = parse_input(&file_content);
    let max_el = *data.iter().max().unwrap();
    let mut helper: HashMap<i32, i32> = HashMap::new();
    let mut current_min = i32::MAX;
    for pos in 0..max_el {
        for curr_el in data.iter() {
            let entry = helper.entry(pos).or_insert(0);
            let diff = (curr_el - pos).abs();
            *entry += (diff * (diff + 1)) / 2;
        }
        current_min = min(current_min, *helper.values().min().unwrap());
    }
    current_min
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 37, 168), ("input.txt", 326132, 88612508)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let res_1 = part_1(&file_content);
        assert_eq!(res_1, result_1);

        let res_2 = part_2(&file_content);
        assert_eq!(res_2, result_2);
    }
    Ok(())
}
