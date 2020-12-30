use std::collections::{HashMap, HashSet};
use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 7, 6), ("input.txt", 535, 212)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();

        let mut substitutes: HashMap<&str, Vec<&str>> = HashMap::new();
        for line in file_content.iter() {
            if line.is_empty() {
                break;
            }
            let splitted: Vec<&str> = line.split(" => ").collect();
            let reference = &splitted[0];
            let replacement = &splitted[1];
            substitutes
                .entry(reference)
                .and_modify(|x| x.push(replacement))
                .or_insert(vec![replacement]);
        }
        let molecule = file_content.last().unwrap();
        let mut uniques: HashSet<String> = HashSet::new();
        for (i, ch) in molecule.chars().enumerate() {
            let (left, right) = if ch.to_lowercase().collect::<Vec<_>>() == vec![ch] {
                (i - 1, i + 1)
            } else {
                (i, i + 1)
            };
            match substitutes.get(&molecule[left..right]) {
                Some(repl) => {
                    for r in repl.iter() {
                        let mut s = String::from(molecule);
                        s.replace_range(left..right, r);
                        uniques.insert(s);
                    }
                }
                None => (),
            }
        }
        assert_eq!(uniques.len(), result_1);

        let reversed: HashMap<&str, &str> = substitutes
            .iter()
            .flat_map(|(&k, v)| v.iter().map(move |&vv| (vv, k)))
            .collect();

        let str_1 = "e";
        let mut str_2 = molecule.to_string();
        let longest_key_len = reversed.keys().map(|x| x.len()).max().unwrap();
        let mut main_offset = 0;
        let mut min_changes = 0;
        while str_1 != str_2 {
            println!("Line remaining: {}, main_offset {}", str_2, main_offset);
            let mut i = 0;
            while i < longest_key_len {
                let offset = if str_2.len() + i >= longest_key_len {
                    str_2.len() + i - longest_key_len
                } else {
                    str_2.len()
                };

                let beginning = if main_offset > offset {
                    0
                } else {
                    offset - main_offset
                };
                let end = str_2.len() - main_offset;
                let tmp_str = &str_2[beginning..end];
                match reversed.get(tmp_str) {
                    Some(val) => {
                        println!("Before {}", str_2);
                        println!("Replaced {} with {}", tmp_str, val);
                        str_2.replace_range(beginning..end, val);
                        println!("After {}", str_2);
                        min_changes += 1;
                        main_offset = 0;
                        break;
                    }
                    None => (),
                }
                i += 1;
            }
            if i == longest_key_len {
                main_offset += 1;
            }
        }
        println!("Min changes: {}", min_changes);
        assert_eq!(min_changes, result_2);
    }
    Ok(())
}
