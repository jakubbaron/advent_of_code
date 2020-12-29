use day_15::{add_spoon, get_score, Spoon};
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

        let mut scores: Vec<i64> = vec![0; spoons_vec[0].as_vec().len()];
        let mut used_spoons = 0;
        for spoon in spoons_vec.iter() {
            scores = add_spoon(&scores, &spoon);
            used_spoons += 1;
        }

        // println!("Score: {}", get_score(&scores));
        // println!("Scores: {:?}", scores);
        // println!("Used Spoons: {:?}", used_spoons);

        let mut res_1 = 0;
        for _ in used_spoons..100 {
            let (score, new_scores) = spoons_vec
                .iter()
                .map(|spoon| add_spoon(&scores, &spoon))
                .map(|new_scores| (get_score(&new_scores), new_scores))
                .max()
                .unwrap();
            scores = new_scores;
            res_1 = score;
        }
        assert_eq!(res_1, result_1);
        let mut calories_scores = vec![vec![(0, vec![0]); 501]; 101];
        let mut scores: Vec<i64> = vec![0; spoons_vec[0].as_vec().len()];
        let mut used_spoons = 0;
        let mut used_kcals = 0;
        for spoon in spoons_vec.iter() {
            scores = add_spoon(&scores, &spoon);
            used_spoons += 1;
            used_kcals += spoon.get_calories();
        }
        calories_scores[used_spoons][used_kcals] = (get_score(&scores), scores);
        for spoons in used_spoons + 1..101 {
            for kcals in used_kcals + 1..501 {
                let mut max_score = 0;
                let mut max_vec: Vec<i64> = Vec::new();
                for spoon in spoons_vec.iter() {
                    if spoon.get_calories() >= kcals {
                        continue;
                    } else {
                        let (_, scores) =
                            &calories_scores[spoons - 1][kcals - spoon.get_calories()];
                        let tmp = add_spoon(&scores, &spoon);
                        let tmp_result = get_score(&tmp);
                        if tmp_result > max_score {
                            max_score = tmp_result;
                            max_vec = tmp;
                        }
                    }
                }
                calories_scores[spoons][kcals] = (max_score, max_vec);
                // println!("[{}][{}]: {:?}", spoons, kcals, calories_scores[spoons][kcals]);
            }
        }
        println!("[{}][{}]: {:?}", 100, 500, calories_scores[100][500]);
        let res_2 = calories_scores[100][500].0;
        assert_eq!(res_2, result_2);
    }
    Ok(())
}
