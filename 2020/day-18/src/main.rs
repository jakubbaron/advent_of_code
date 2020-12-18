use std::io::{self};

fn evaluate(characters: &Vec<char>) -> (i64, usize) {
    let mut results = 0;
    let mut id = 0;
    let mut operator = '+';
    while id < characters.len() {
        let ch = characters[id];
        id += 1;
        if ch == '(' {
            let (res, offset) = evaluate(&characters[id..characters.len()].to_vec());
            if operator == '*' {
                results *= res;
            } else if operator == '+' {
                results += res;
            }
            id += offset;
        } else if ch == ')' {
            println!("Bracket, Returning: {} {}", results, id);
            return (results, id);
        } else if ch == '*' || ch == '+' {
            operator = ch;
        } else {
            let left = ch.to_digit(10).unwrap() as i64;
            println!("Res: {}, left: {}, op: {}", results, left, operator);
            if operator == '*' {
                results *= left;
            } else if operator == '+' {
                results += left;
            }
        }
    }
    println!("Returning: {} {}", results, id);
    (results, id)
}

fn evaluate2(characters: &Vec<char>) -> (i64, usize) {
    let mut results = 0;
    let mut id = 0;
    while id < characters.len() {
        let ch = characters[id];
        id += 1;
        if ch == '(' {
            let (res, offset) = evaluate2(&characters[id..characters.len()].to_vec());
            results += res;
            id += offset;
        } else if ch == ')' {
            println!("Bracket, Returning: {} {}", results, id);
            return (results, id);
        } else if ch == '+' {
            continue;
        } else if ch == '*' {
            let (res, offset) = evaluate2(&characters[id..characters.len()].to_vec());
            results *= res;
            id += offset;
            println!("Multi, Returning: {} {}", results, id);
            return (results, id);
        } else {
            let right = ch.to_digit(10).unwrap() as i64;
            results += right;
        }
    }
    println!("Returning: {} {}", results, id);
    (results, id)
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 71_i64, 231_i64),
        ("test2.txt", 51, 51),
        ("test3.txt", 437, 1445),
        ("test4.txt", 12240, 669060),
        ("test5.txt", 13632, 23340),
        ("input.txt", 5019432542701, 70518821989947),
    ];
    for (f, result_1, result_2) in files_results.iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut sum = 0;
        for line in file_content.iter() {
            let characters: Vec<char> = line.chars().filter(|x| *x != ' ').collect();
            let (res, _) = evaluate(&characters);
            sum += res;
        }
        println!("Sum: {}", sum);
        assert_eq!(sum, *result_1);

        let mut sum = 0;
        for line in file_content.iter() {
            let characters: Vec<char> = line.chars().filter(|x| *x != ' ').collect();
            let (res, _) = evaluate2(&characters);
            sum += res;
        }
        assert_eq!(sum, *result_2);
    }
    Ok(())
}
