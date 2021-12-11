use std::collections::HashMap;
use std::io::{self};

fn part_1(file_content: &Vec<String>) -> usize {
    let scores: HashMap<char, usize> = vec![(')', 3), (']', 57), ('}', 1197), ('>', 25137)]
        .into_iter()
        .collect();
    let brackets: HashMap<char, char> = vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect();

    let mut score: usize = 0;
    for line in file_content.iter() {
        let mut stack: Vec<char> = vec![];
        for ch in line.chars() {
            if brackets.contains_key(&ch) {
                stack.push(ch);
            } else {
                if stack.is_empty() {
                    score += scores[&ch];
                    break;
                } else {
                    let last_opening = stack.pop().unwrap();
                    if brackets[&last_opening] != ch {
                        score += scores[&ch];
                        break;
                    }
                }
            }
        }
    }
    score
}

fn part_2(file_content: &Vec<String>) -> usize {
    let scores: HashMap<char, usize> = vec![(')', 1), (']', 2), ('}', 3), ('>', 4)]
        .into_iter()
        .collect();
    let brackets: HashMap<char, char> = vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .into_iter()
        .collect();

    let mut line_scores: Vec<usize> = vec![];
    for line in file_content.iter() {
        let mut is_valid: bool = true;
        let mut stack: Vec<char> = vec![];
        for ch in line.chars() {
            if brackets.contains_key(&ch) {
                stack.push(ch);
            } else {
                if stack.is_empty() {
                    is_valid = false;
                    break;
                } else {
                    let last_opening = stack.pop().unwrap();
                    if brackets[&last_opening] != ch {
                        is_valid = false;
                        break;
                    }
                }
            }
        }
        if is_valid {
            let mut temp_score = 0;
            for ch in stack.iter().rev() {
                temp_score *= 5;
                temp_score += scores[&brackets[&ch]];
            }
            line_scores.push(temp_score);
        }
    }
    line_scores.sort();
    line_scores[line_scores.len() / 2]
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 26397, 288957), ("input.txt", 339411, 0)];
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
