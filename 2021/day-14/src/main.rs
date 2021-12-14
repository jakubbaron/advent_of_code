use std::collections::HashMap;
use std::io::{self};

fn parse_input(file_content: &Vec<String>) -> (String, HashMap<&str, &str>) {
    let index = file_content.iter().position(|x| x.is_empty()).unwrap();
    let polymer = file_content[0].to_string();
    let mut mapping: HashMap<_, _> = HashMap::new();
    for line in file_content[index + 1..].iter() {
        let splitted = line.split(" -> ").collect::<Vec<_>>();
        mapping.insert(splitted[0], splitted[1]);
    }
    (polymer, mapping)
}
fn count_occurences(polymer: &String) -> usize {
    let mut counter: HashMap<_, _> = HashMap::new();
    for ch in polymer.chars() {
        let entry = counter.entry(ch).or_insert(0);
        *entry += 1;
    }
    let min = counter.values().min().unwrap();
    let max = counter.values().max().unwrap();
    max - min
}

fn part_1(file_content: &Vec<String>) -> usize {
    let (mut polymer, mapping) = parse_input(&file_content);
    for _ in 0..10 {
        let mut collector: Vec<_> = vec![];
        for idx in 0..(polymer.len() - 1) {
            collector.push(&polymer[idx..idx + 1]);
            collector.push(mapping[&polymer[idx..idx + 2]]);
        }
        collector.push(&polymer[polymer.len() - 1..polymer.len()]);
        polymer = collector.iter().map(|x| x.to_string()).collect::<String>();
    }
    count_occurences(&polymer)
}

fn part_2(file_content: &Vec<String>) -> usize {
    0
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 1588, 2188189693529), ("input.txt", 2027, 0)];
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
