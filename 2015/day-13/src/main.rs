use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::io::{self};

fn get_happiness(names: &Vec<&str>, scores: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let mut total_score = 0;
    for i in 0..names.len() {
        let left_neighbour = if i > 0 {
            names[i - 1]
        } else {
            names.last().unwrap()
        };
        let right_neighbour = if i < names.len() - 1 {
            names[i + 1]
        } else {
            names.first().unwrap()
        };
        let person_scores = scores.get(names[i]).unwrap();
        total_score += person_scores.get(left_neighbour).unwrap();
        total_score += person_scores.get(right_neighbour).unwrap();
    }
    total_score
}

fn permutations(mut vec: Vec<&str>, l: usize, r: usize) -> Vec<Vec<&str>> {
    let mut vecs: Vec<Vec<&str>> = Vec::new();
    if r == l {
        return vec![vec];
    } else {
        for i in l..=r {
            let tmp = vec[l];
            vec[l] = vec[i];
            vec[i] = tmp;

            vecs.extend(permutations(vec.to_vec(), l+1, r));

            let tmp = vec[l];
            vec[l] = vec[i];
            vec[i] = tmp;
        }
    }
    vecs
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 330, 286), ("input.txt", 664, 640)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut happiness: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
        let re = Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).")
            .unwrap();

        for line in file_content.iter() {
            let caps = re.captures(&line).unwrap();
            let name_1 = caps.get(1).map_or("", |m| m.as_str());
            let name_2 = caps.get(4).map_or("", |m| m.as_str());
            let cmd = caps.get(2).map_or("", |m| m.as_str());
            let modifier = if cmd == "gain" { 1 } else { -1 };
            let number = caps
                .get(3)
                .map_or("", |m| m.as_str())
                .parse::<i32>()
                .unwrap()
                * modifier;
            happiness
                .entry(&name_1)
                .and_modify(|x| {
                    x.insert(name_2, number);
                })
                .or_insert(
                    vec![(name_2, number), ("myself", 0)]
                        .into_iter()
                        .collect::<HashMap<&str, i32>>(),
                );
        }

        let names: Vec<&str> = happiness.keys().cloned().collect();
        let mut permuted = permutations(names[1..].to_vec(), 0, names.len() - 2);
        for row in &mut permuted {
            row.push(names[0]);
        }
        let mut max_score = 0;
        for perm in permuted.iter() {
            max_score = max(max_score, get_happiness(&perm, &happiness));
        }
        assert_eq!(max_score, result_1);

        happiness.entry("myself").or_insert_with(HashMap::new);
        for name in names.iter() {
            if name != &"myself" {
                happiness.entry("myself").and_modify(|x| {
                    x.insert(name, 0);
                });
            }
        }
        let names: Vec<&str> = happiness.keys().cloned().collect();
        let mut permuted = permutations(names[1..].to_vec(), 0, names.len() - 2);
        for row in &mut permuted {
            row.push(names[0]);
        }
        let mut max_score = 0;
        for perm in permuted.iter() {
            max_score = max(max_score, get_happiness(&perm, &happiness));
        }
        assert_eq!(max_score, result_2);
    }
    Ok(())
}
