use std::collections::HashMap;
use std::io::{self};

fn parse_input(file_content: &Vec<String>) -> (String, HashMap<&str, char>) {
    let index = file_content.iter().position(|x| x.is_empty()).unwrap();
    let polymer = file_content[0].to_string();
    let mut mapping: HashMap<_, _> = HashMap::new();
    for line in file_content[index + 1..].iter() {
        let splitted = line.split(" -> ").collect::<Vec<_>>();
        mapping.insert(splitted[0], splitted[1].chars().nth(0).unwrap());
    }
    (polymer, mapping)
}

fn expand_polymer(
    s: &str,
    mapping: &HashMap<&str, char>,
    cache: &mut HashMap<(usize, String), HashMap<char, usize>>,
    depth: usize,
    max_depth: usize,
) -> HashMap<char, usize> {
    if depth == max_depth {
        // We only store the second character not to double count
        // e.g. if we come here while expanding NN, that would result
        // in NCN, left entry here would store C and right entry will
        // store N. So the only problem occur for the very first entry
        // of the polymer what is being fixed in expand_n_times
        let letter_2 = s.chars().nth(1).unwrap();
        return vec![(letter_2, 1)].into_iter().collect();
    }

    let new_letter = mapping[s];
    let left = format!("{}{}", s.chars().nth(0).unwrap(), new_letter);
    let right = format!("{}{}", new_letter, s.chars().nth(1).unwrap());

    let left_key = (depth + 1, left.to_string());
    let left_cache = if cache.contains_key(&left_key) {
        cache[&left_key].clone()
    } else {
        expand_polymer(&left, &mapping, cache, depth + 1, max_depth)
    };
    let right_key = (depth + 1, right.to_string());
    let right_cache = if cache.contains_key(&right_key) {
        cache[&right_key].clone()
    } else {
        expand_polymer(&right, &mapping, cache, depth + 1, max_depth)
    };

    let mut merged_caches: HashMap<char, usize> = left_cache.clone();
    for (k, v) in right_cache.iter() {
        let entry = merged_caches.entry(*k).or_insert(0);
        *entry += *v;
    }
    let main_key = (depth, s.to_string());
    cache.insert(main_key, merged_caches.clone());
    merged_caches
}

fn expand_n_times(polymer: &String, mapping: &HashMap<&str, char>, n: usize) -> usize {
    let mut cache: HashMap<_, _> = HashMap::new();
    for i in 0..polymer.len() - 1 {
        let _ = expand_polymer(&polymer[i..i + 2], mapping, &mut cache, 0, n);
    }

    let mut counter: HashMap<_, _> = HashMap::new();
    for i in 0..polymer.len() - 1 {
        let key = (0, polymer[i..i + 2].to_string());
        let temp_map = &cache[&key];
        for (k, v) in temp_map.iter() {
            let entry = counter.entry(*k).or_insert(0);
            *entry += *v;
        }
    }

    let first_char = &polymer[0..1].chars().nth(0).unwrap();
    let entry = counter.entry(*first_char).or_insert(0);
    *entry += 1;

    let min = counter.values().min().unwrap();
    let max = counter.values().max().unwrap();
    max - min
}
fn part_1(file_content: &Vec<String>) -> usize {
    let (polymer, mapping) = parse_input(&file_content);
    expand_n_times(&polymer, &mapping, 10)
}

fn part_2(file_content: &Vec<String>) -> usize {
    let (polymer, mapping) = parse_input(&file_content);
    expand_n_times(&polymer, &mapping, 40)
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 1588, 2188189693529),
        ("input.txt", 2027, 2265039461737),
    ];
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
