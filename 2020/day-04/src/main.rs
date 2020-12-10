use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn check_map(m: &HashMap<String, String>) -> bool {
    for (key, value) in m.iter() {
        match &key[..] {
            "byr" => {
                let my_int = value.parse::<i32>().unwrap();
                if my_int < 1920 || my_int > 2002 {
                    return false;
                }
            }
            "iyr" => {
                let my_int = value.parse::<i32>().unwrap();
                if my_int < 2010 || my_int > 2020 {
                    return false;
                }
            }
            "eyr" => {
                let my_int = value.parse::<i32>().unwrap();
                if my_int < 2020 || my_int > 2030 {
                    return false;
                }
            }
            "hgt" => {
                let re = Regex::new(r"(\d+)(in|cm)").unwrap();
                if !re.is_match(value) {
                    return false;
                }
                let caps = re.captures(value).unwrap();
                let my_int = caps
                    .get(1)
                    .map_or("", |m| m.as_str())
                    .parse::<i32>()
                    .unwrap();
                let unit = caps.get(2).map_or("", |m| m.as_str());
                if unit == "in" {
                    if my_int < 59 || my_int > 76 {
                        return false;
                    }
                } else if unit == "cm" {
                    if my_int < 150 || my_int > 193 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            "hcl" => {
                let re = Regex::new(r"^#[a-z0-9]{6}$").unwrap();
                if !re.is_match(value) {
                    return false;
                }
            }
            "ecl" => {
                let re = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
                if !re.is_match(value) {
                    return false;
                }
            }
            "pid" => {
                let re = Regex::new(r"^\d{9}$").unwrap();
                if !re.is_match(value) {
                    return false;
                }
            }
            _ => (),
        }
    }
    true
}
fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let expected: HashSet<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let mut counter_1 = 0;
    let mut counter_2 = 0;
    let mut received: HashSet<String> = HashSet::new();
    let mut m: HashMap<String, String> = HashMap::new();
    let mut vec: Vec<String> = Vec::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        vec.push(my_string);
    }
    for my_string in &vec {
        let field_values: Vec<&str> = my_string.split(" ").collect();
        for fv in field_values {
            let tmp: Vec<&str> = fv.split(":").collect();
            if tmp.len() != 2 {
                continue;
            }
            let field = String::from(tmp[0]);
            let value = String::from(tmp[1]);
            if !field.is_empty() {
                received.insert(String::from(&field));
                m.insert(field, value);
            }
        }
        if my_string.is_empty() {
            if received.is_superset(&expected) {
                counter_1 += 1;
                if check_map(&m) {
                    counter_2 += 1;
                }
            }
            received.clear();
            m.clear();
        }
    }

    if !received.is_empty() {
        if received.is_superset(&expected) {
            counter_1 += 1;
            if check_map(&m) {
                counter_2 += 1;
            }
        }
        received.clear();
    }

    println!("Valid passports {}", counter_1);
    println!("Valid passports {}", counter_2);

    Ok(())
}
