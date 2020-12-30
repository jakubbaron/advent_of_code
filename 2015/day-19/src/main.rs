use std::collections::{HashMap, HashSet};
use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 7, 1),
        ("input.txt", ,535 1)
    ];
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
    }
    Ok(())
}
