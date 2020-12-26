use std::io::{self};
use regex::Regex;
use std::cmp::max;

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 1_000 * 1_000, 1_000 * 1_000),
        ("test2.txt", 1_000, 2_000),
        ("test3.txt", 0, 0),
        ("test4.txt", 1_000 * 1_000 - 1_000 - 4, 1_000 * 1_000 + 2_000 - 4),
        ("input.txt", 569999, 17836115)
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut lights = vec![vec![false; 1000]; 1000];
        let mut brightness = vec![vec![0; 1000]; 1000];
        let re = Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        for line in file_content.iter() {
            let caps = re.captures(&line).unwrap();
            let command = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let start_x = caps.get(2).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
            let start_y = caps.get(3).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
            let end_x = caps.get(4).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
            let end_y = caps.get(5).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
            for x in start_x..=end_x {
                for y in start_y..=end_y {
                    if command == "turn on" {
                        lights[x][y] = true;
                        brightness[x][y] += 1;
                    } else if command == "toggle" {
                        lights[x][y] = !lights[x][y];
                        brightness[x][y] += 2;
                    } else if command == "turn off" {
                        lights[x][y] = false;
                        brightness[x][y] = max(brightness[x][y] - 1, 0);
                    } else {
                        panic!("Unrecognized command: {}", command);
                    }
                }
            }
        }
        assert_eq!(lights.iter().flat_map(|row| row.iter()).filter(|&&light| light).count(), result_1);
        assert_eq!(brightness.iter().flat_map(|row| row.iter()).fold(0, |acc, val| acc + val), result_2);
    }
    Ok(())
}
