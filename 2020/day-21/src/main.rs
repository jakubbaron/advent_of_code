use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 5, "mxmxvkd,sqjhc,fvjkl"),
        (
            "input.txt",
            2485,
            "bqkndvb,zmb,bmrmhm,snhrpv,vflms,bqtvr,qzkjrtl,rkkrx",
        ),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let re = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
        let mut allergens: HashMap<String, HashSet<String>> =
            HashMap::with_capacity(file_content.len());
        let mut all_sets: Vec<HashSet<String>> = Vec::new();
        let mut all_elvish: HashSet<String> = HashSet::new();
        for line in file_content.into_iter() {
            let splitted = re.captures(&line).unwrap();
            let elvish = splitted.get(1).map_or("", |m| m.as_str()).to_string();
            let english = splitted.get(2).map_or("", |m| m.as_str()).to_string();
            let english = english.replace(",", "");

            let elvish: HashSet<String> = elvish.split(" ").map(|m| m.to_string()).collect();
            let english: HashSet<String> = english.split(" ").map(|m| m.to_string()).collect();
            for eng_word in english.iter() {
                allergens
                    .entry(eng_word.to_string())
                    .or_insert(elvish.clone());
                let entry = allergens.get(eng_word).unwrap();
                let inter: HashSet<String> = entry.intersection(&elvish).cloned().collect();
                allergens.insert(eng_word.to_string(), inter);
            }
            all_elvish.extend(elvish.clone());
            all_sets.push(elvish.clone());
        }
        for (k, v) in allergens.iter() {
            println!("{} {:?}", k, v);
        }
        for val in allergens.values() {
            all_elvish = all_elvish.difference(&val).cloned().collect();
        }
        // println!("{:?} -> {}", all_elvish, all_elvish.len());
        let mut res_1 = 0;
        for set in all_sets.iter() {
            let inter: HashSet<_> = set.intersection(&all_elvish).collect();
            res_1 += inter.len();
        }

        assert_eq!(res_1, result_1);
        let mut words: Vec<(String, HashSet<String>)> = allergens.clone().into_iter().collect();
        for i in 0..words.len() {
            words.sort_by_key(|(_k, val)| val.len());
            let (_, set) = words[i].clone();
            let to_remove = set.iter().next().unwrap();
            for j in i + 1..words.len() {
                words[j].1.remove(to_remove);
            }
        }
        assert!(words.iter().all(|(_, v)| v.len() == 1));
        words.sort_by_key(|k| k.0.to_string());
        let final_list: Vec<String> = words
            .into_iter()
            .map(|(_, val)| val.into_iter().next().unwrap())
            .collect();
        let joined = final_list.join(",");
        assert_eq!(joined, result_2);
    }
    Ok(())
}
