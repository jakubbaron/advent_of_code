use regex::Regex;
use serde_json::{Result, Value};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::{self};

fn count_numbers(line: &str) -> i64 {
    let re = Regex::new(r"([-]?\d+)").unwrap();
    re.find_iter(&line)
        .map(|c| c.as_str().parse::<i64>().unwrap())
        .fold(0, |acc, val| acc + val)
}

fn counter(vals: &Value, skip_red: bool) -> i64 {
    match vals {
        Value::Object(map) => {
            if !skip_red {
                for (k, v) in map {
                    if v == &Value::String("red".to_string()) {
                        return 0;
                    }
                }
            }
            let mut sum = 0;
            for (k, v) in map {
                sum += counter(v, skip_red);
            }
            return sum;
        }
        Value::Array(array) => {
            return array
                .iter()
                .map(|v| counter(v, skip_red))
                .fold(0, |acc, val| acc + val);
        }
        Value::Number(n) => {
            return n.as_i64().unwrap();
        }
        _ => {
            return 0;
        }
    }
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 51, 49),
        ("test3.txt", 2942, 0),
        ("test4.txt", 6, 6),
        ("input.txt", 111754, 65402),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let line = &file_content[0];
        assert_eq!(count_numbers(&line), result_1);

        let v: Value = serde_json::from_str(line).unwrap();
        assert_eq!(counter(&v, true), result_1);
        assert_eq!(counter(&v, false), result_2);
    }
    Ok(())
}
