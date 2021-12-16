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
fn part_1(file_content: &Vec<String>) -> i32 {
    let risks = parse_input(&file_content);
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

fn part_2(file_content: &Vec<String>) -> i32 {
    0
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 40, 0), ("input.txt", 673, 0)];
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
