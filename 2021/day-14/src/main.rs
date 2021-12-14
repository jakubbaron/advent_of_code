use std::collections::HashMap;
use std::io::{self};

fn parse_input(file_content: &Vec<String>) -> (String, HashMap<String, char>) {
    let index = file_content.iter().position(|x| x.is_empty()).unwrap();
    let polymer = file_content[0].to_string();
    let mut mapping: HashMap<_, _> = HashMap::new();
    for line in file_content[index + 1..].iter() {
        let splitted = line.split(" -> ").collect::<Vec<_>>();
        mapping.insert(splitted[0].to_string(), splitted[1].chars().nth(0).unwrap());
    }
    (polymer, mapping)
}

fn update_counter(polymer: &str, counter: &mut HashMap<char, usize>) {
    for ch in polymer[1..].chars() {
        let entry = counter.entry(ch).or_insert(0);
        *entry += 1;
    }
}

fn expand_polymer(
    s: &str,
    mapping: &HashMap<String, char>,
    counter: &mut HashMap<char, usize>,
    depth: usize,
    max_depth: usize,
) {
    if depth == max_depth {
        update_counter(s, counter);
        return;
    }
    let new_letter = mapping[s];
    let left = format!("{}{}", s.chars().nth(0).unwrap(), new_letter);
    let right = format!("{}{}", new_letter, s.chars().nth(1).unwrap());
    expand_polymer(&left, mapping, counter, depth + 1, max_depth);
    expand_polymer(&right, mapping, counter, depth + 1, max_depth);
}

fn part_1(file_content: &Vec<String>) -> usize {
    let (polymer, mapping) = parse_input(&file_content);
    let mut counter: HashMap<_, _> = HashMap::new();
    for i in 0..polymer.len() - 1 {
        expand_polymer(&polymer[i..i + 2], &mapping, &mut counter, 0, 10)
    }

    for (k, v) in counter.iter() {
        println!("{} {}", k, v);
    }

    let first_char = &polymer[0..1].chars().nth(0).unwrap();
    let entry = counter.entry(*first_char).or_insert(0);
    *entry += 1;

    let min = counter.values().min().unwrap();
    let max = counter.values().max().unwrap();
    max - min
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
