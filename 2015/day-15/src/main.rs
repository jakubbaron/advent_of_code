use day_15::{optimize_for_score, optimize_for_score_and_kcals, Spoon};
use regex::Regex;
use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 62842880, 57600000),
        ("input.txt", 18965440, 15862900),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let re = Regex::new(r"(\w+): capacity ([-]?\d+), durability ([-]?\d+), flavor ([-]?\d+), texture ([-]?\d+), calories ([-]?\d+)").unwrap();
        let mut spoons_vec: Vec<Spoon> = Vec::new();
        for line in file_content.iter() {
            let caps = re.captures(&line).unwrap();
            let name = caps.get(1).map_or("", |m| m.as_str());
            let capacity = caps
                .get(2)
                .map_or("", |m| m.as_str())
                .parse::<i64>()
                .unwrap();
            let durability = caps
                .get(3)
                .map_or("", |m| m.as_str())
                .parse::<i64>()
                .unwrap();
            let flavor = caps
                .get(4)
                .map_or("", |m| m.as_str())
                .parse::<i64>()
                .unwrap();
            let texture = caps
                .get(5)
                .map_or("", |m| m.as_str())
                .parse::<i64>()
                .unwrap();
            let calories = caps
                .get(6)
                .map_or("", |m| m.as_str())
                .parse::<usize>()
                .unwrap();
            spoons_vec.push(Spoon::new(
                name, capacity, durability, flavor, texture, calories,
            ));
        }

        assert_eq!(optimize_for_score(&spoons_vec, 100), result_1);
        assert_eq!(
            optimize_for_score_and_kcals(&spoons_vec, 500, 100),
            result_2
        );
    }
    Ok(())
}
