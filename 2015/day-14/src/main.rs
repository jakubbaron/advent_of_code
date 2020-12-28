use day_14::Reindeer;
use regex::Regex;
use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 2660, 1), ("input.txt", 2655, 1)];
    for (f, result_1, _result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let re = Regex::new(
            r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
        )
        .unwrap();
        let mut reindeers: Vec<Reindeer> = Vec::new();
        for line in file_content.iter() {
            let caps = re.captures(&line).unwrap();
            let _name = caps.get(1).map_or("", |m| m.as_str());
            let speed = caps
                .get(2)
                .map_or("", |m| m.as_str())
                .parse::<u32>()
                .unwrap();
            let run_time = caps
                .get(3)
                .map_or("", |m| m.as_str())
                .parse::<u32>()
                .unwrap();
            let rest_time = caps
                .get(4)
                .map_or("", |m| m.as_str())
                .parse::<u32>()
                .unwrap();
            reindeers.push(Reindeer::new(speed, run_time, rest_time));
        }
        let mut times: Vec<u32> = Vec::new();
        for reindeer in reindeers.iter() {
            times.push(reindeer.run_for(2503));
        }
        let res_1 = *times.iter().max().unwrap();
        println!("Winning distance: {}", res_1);
        assert_eq!(res_1, result_1);
    }
    Ok(())
}
