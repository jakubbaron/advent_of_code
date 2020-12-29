use std::io::{self};
use regex::Regex;
use day_15::{Spoon, add_spoon, get_score};

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 62842880, 1),
        ("input.txt", 18965440, 1)
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let re = Regex::new(r"(\w+): capacity ([-]?\d+), durability ([-]?\d+), flavor ([-]?\d+), texture ([-]?\d+), calories ([-]?\d+)").unwrap();
        let mut spoons: Vec<Spoon> = Vec::new();
        for line in file_content.iter() {
            let caps = re.captures(&line).unwrap();
            let name = caps.get(1).map_or("", |m| m.as_str());
            let capacity = caps.get(2).map_or("", |m| m.as_str()).parse::<i64>().unwrap();
            let durability = caps.get(3).map_or("", |m| m.as_str()).parse::<i64>().unwrap();
            let flavor = caps.get(4).map_or("", |m| m.as_str()).parse::<i64>().unwrap();
            let texture = caps.get(5).map_or("", |m| m.as_str()).parse::<i64>().unwrap();
            let calories = caps.get(6).map_or("", |m| m.as_str()).parse::<i64>().unwrap();
            spoons.push(Spoon::new(name, capacity, durability, flavor, texture, calories));
        }

        let mut scores: Vec<i64> = vec![0;spoons[0].as_vec().len()];
        let mut used_spoons = 0;
        for spoon in spoons.iter() {
            scores = add_spoon(&scores, &spoon);
            used_spoons += 1;
        }

        // println!("Score: {}", get_score(&scores));
        // println!("Scores: {:?}", scores);
        // println!("Used Spoons: {:?}", used_spoons);

        let mut res_1 = 0;
        while used_spoons < 100 {
            used_spoons += 1;
            let (score, new_scores) = spoons.iter().map(|spoon| add_spoon(&scores, &spoon)).map(|new_scores| (get_score(&new_scores), new_scores)).max().unwrap();
            // println!("Used spoons: {} score: {}", used_spoons, score);
            scores = new_scores;
            res_1 = score;
        }
        assert_eq!(res_1, result_1);
    }
    Ok(())
}
