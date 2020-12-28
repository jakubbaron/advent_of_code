use std::io::{self};
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;

fn count_numbers(line: &str) -> i32 {
    let re = Regex::new(r"([-]?\d+)").unwrap();
    re.find_iter(&line).map(|c| {
        c.as_str().parse::<i32>().unwrap()
    }).fold(0, |acc, val| acc + val)
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 51, 49),
        ("test3.txt", 2942, 0),
        ("test4.txt", 6, 6),
        ("input.txt", 111754, 65402)
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let line = &file_content[0];
        // assert_eq!(count_numbers(&line), result_1);

        let chars: Vec<char> = line.chars().collect();
        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        let mut openings: Vec<usize> = Vec::new();
        let mut id = 0;
        while id < chars.len() {
            if chars[id] == '{' {
                openings.push(id);
            } else if chars[id] == '}' {
                to_remove.push((openings.pop().unwrap(), id));
            }
            id += 1;
        }
        let red_str = "\":\"red\"";
        let mut having_red: Vec<(usize, usize)> = Vec::new();
        for (open, close) in to_remove.iter() {
            let slice = &line[*open..*close+1];
            if !slice.contains(red_str) {
                continue;
            }

            let mut replace_index: Option<usize> = None;
            for (id, (o2, c2)) in having_red.iter().enumerate() {
                if max(close, c2) - min(open, o2) < (close - open) + (c2 - o2) {
                    let mut all_sets: HashSet<usize> = HashSet::new();
                    for (o, c) in having_red.iter() {
                        if o == o2 && c == c2 {
                            continue;
                        }
                        let prev: HashSet<_> = line[*o..*c+1].match_indices(red_str).map(|(val, _)| val+*o).collect();
                        all_sets = all_sets.union(&prev).cloned().collect();
                    }
                    let v: HashSet<_> = slice.match_indices(red_str).map(|(val, _)| val + *open).collect();
                    let previous: HashSet<_> = line[*o2..*c2+1].match_indices(red_str).map(|(val, _)| val+*o2).collect();
                    if v.is_superset(&previous) && v.is_disjoint(&all_sets) {
                        replace_index = Some(id);
                        break;
                    }
                }
            }
            if let Some(i) = replace_index {
                println!("Replacing {:?} with ({}, {})", having_red[i], open, close);
                having_red[i] = (*open, *close);
            } else {
                let v: HashSet<_> = slice.match_indices(red_str).map(|(val, _)| val + *open).collect();
                let mut all_sets: HashSet<usize> = HashSet::new();
                for (o, c) in having_red.iter() {
                    let previous: HashSet<_> = line[*o..*c+1].match_indices(red_str).map(|(val, _)| val+*o).collect();
                    all_sets = all_sets.union(&previous).cloned().collect();
                }
                if v.is_disjoint(&all_sets) {
                    println!("Pushing {} {}", open, close);
                    having_red.push((*open, *close));
                }
            }
        }
        println!("{:?}", having_red);

        let mut red_sum = 0;
        for (open, close) in having_red.iter() {
            red_sum += count_numbers(&line[*open..*close+1]);
            // println!("{}", &line[*open..*close+1]);
        }

        println!("Red sum: {}", red_sum);
        let s = count_numbers(&line) - red_sum;
        println!("Sum: {}", s);
        assert_eq!(s, result_2);
    }
    Ok(())
}
