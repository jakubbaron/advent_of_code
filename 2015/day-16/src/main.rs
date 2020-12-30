use std::collections::HashMap;
use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![("input.txt", 103, 1)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut sues: HashMap<usize, HashMap<&str, usize>> = HashMap::new();
        for line in file_content.iter() {
            let pos = line.find(':').unwrap();
            let sue_no = line[0..pos].replace("Sue ", "").parse::<usize>().unwrap();
            sues.insert(sue_no, HashMap::new());
            let sue_characteristics: Vec<Vec<&str>> = line[pos + 2..]
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.split(": ").collect())
                .collect();
            for chars in sue_characteristics.into_iter() {
                let ch = &chars[0];
                let no = chars[1].parse::<usize>().unwrap();
                sues.entry(sue_no).and_modify(|x| {
                    x.insert(ch, no);
                });
            }
        }
        let my_sue: HashMap<&str, usize> = vec![
            ("children", 3),
            ("cats", 7),
            ("samoyeds", 2),
            ("pomeranians", 3),
            ("akitas", 0),
            ("vizslas", 0),
            ("goldfish", 5),
            ("trees", 3),
            ("cars", 2),
            ("perfumes", 1),
        ]
        .into_iter()
        .collect();
        let mut res_1 = 0;
        for (sue_no, sue_chars) in sues.iter() {
            let mut it_is_my_sue = true;
            for (char_name, char_value) in sue_chars.iter() {
                if my_sue.get(char_name).unwrap() != char_value {
                    it_is_my_sue = false;
                    break;
                }
            }
            if it_is_my_sue {
                res_1 = *sue_no;
                break;
            }
        }
        assert_eq!(res_1, result_1);

        let mut res_2 = 0;
        for (sue_no, sue_chars) in sues.iter() {
            let mut it_is_my_sue = true;
            for (char_name, char_val) in sue_chars.iter() {
                let my_sue_val = my_sue.get(char_name).unwrap();
                if char_name == &"cats" || char_name == &"trees" {
                    if my_sue_val > char_val {
                        it_is_my_sue = false;
                        break;
                    }
                } else if char_name == &"pomeranians" || char_name == &"goldfish" {
                    if my_sue_val < char_val {
                        it_is_my_sue = false;
                        break;
                    }
                } else {
                    if my_sue_val != char_val {
                        it_is_my_sue = false;
                        break;
                    }
                }
            }
            if it_is_my_sue {
                res_2 = *sue_no;
                break;
            }
        }
        assert_eq!(res_2, result_2);
    }
    Ok(())
}
