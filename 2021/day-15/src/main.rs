use std::io::{self};

fn parse_input(file_content: &Vec<String>) -> Vec<Vec<i32>> {
    file_content
        .iter()
        .map(|row| {
            row.chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn navigate_with_lowest_risk(risks: &Vec<Vec<i32>>) -> i32 {
    let mut lowest_risks = vec![vec![i32::MAX; risks[0].len()]; risks.len()];
    lowest_risks[0][0] = 0;
    for i in 0..risks.len() {
        for j in 0..risks[i].len() {
            if i == 0 && j == 0 {
                continue;
            }
            let mut current_risks = vec![];
            if i > 0 {
                current_risks.push(lowest_risks[i - 1][j]);
            }
            if j > 0 {
                current_risks.push(lowest_risks[i][j - 1])
            }
            lowest_risks[i][j] = risks[i][j] + current_risks.iter().min().unwrap();
        }
    }

    lowest_risks[lowest_risks.len() - 1][lowest_risks[0].len() - 1]
}

fn navigate_twice(risks: &Vec<Vec<i32>>) -> i32 {
    let mut lowest_risks = vec![vec![i32::MAX; risks[0].len()]; risks.len()];
    lowest_risks[0][0] = 0;
    for i in 0..risks.len() {
        for j in 0..risks[i].len() {
            if i == 0 && j == 0 {
                continue;
            }
            let mut current_risks = vec![];
            if i > 0 {
                current_risks.push(lowest_risks[i - 1][j]);
            }
            if j > 0 {
                current_risks.push(lowest_risks[i][j - 1])
            }
            lowest_risks[i][j] = risks[i][j] + current_risks.iter().min().unwrap();
        }
    }

    let mut curr_min = lowest_risks[lowest_risks.len() - 1][lowest_risks[0].len() - 1];
    let mut last_min = i32::MAX;
    while curr_min != last_min {
        // println!("{} {}", curr_min, last_min);
        curr_min = last_min;
        for i in 0..risks.len() {
            for j in 0..risks[i].len() {
                if i == 0 && j == 0 {
                    continue;
                }
                let mut current_risks = vec![];
                if i > 0 {
                    current_risks.push(lowest_risks[i - 1][j]);
                }
                if j > 0 {
                    current_risks.push(lowest_risks[i][j - 1])
                }
                if i < lowest_risks.len() - 1 {
                    current_risks.push(lowest_risks[i + 1][j]);
                }
                if j < lowest_risks[i].len() - 1 {
                    current_risks.push(lowest_risks[i][j + 1]);
                }
                lowest_risks[i][j] = risks[i][j] + current_risks.iter().min().unwrap();
            }
        }
        last_min = lowest_risks[lowest_risks.len() - 1][lowest_risks[0].len() - 1];
    }

    last_min
}

fn part_1(file_content: &Vec<String>) -> i32 {
    let risks = parse_input(&file_content);
    navigate_with_lowest_risk(&risks)
}

fn part_2(file_content: &Vec<String>) -> i32 {
    let risks = parse_input(&file_content);
    let mut bigger_risks = vec![Vec::with_capacity(risks[0].len() * 5); risks.len() * 5];
    for i in 0..risks.len() {
        let row = risks[i].to_vec();
        for j in 0..5 {
            let mutated_row = row
                .iter()
                .map(|x| if (x + j) > 9 { x + j - 9 } else { x + j });
            bigger_risks[i].extend(mutated_row)
        }
    }
    for i in 0..risks.len() {
        for j in 1..5 {
            let jj = j as i32;
            let row = bigger_risks[i].to_vec();
            let mutated_row = row
                .iter()
                .map(|x| if (x + jj) > 9 { x + jj - 9 } else { x + jj })
                .collect();
            bigger_risks[i + j * risks.len()] = mutated_row;
        }
    }
    navigate_twice(&bigger_risks)
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 40, 315), ("input.txt", 673, 2893)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let res_1 = part_1(&file_content);
        assert_eq!(res_1, result_1);

        let res_2 = part_2(&file_content);
        assert_eq!(res_2, result_2);
    }
    Ok(())
}
