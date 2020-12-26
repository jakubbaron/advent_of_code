use std::io::{self};

fn get_result_1(file_content: &Vec<String>) -> usize {
    let line = &file_content[0];
    line.chars().filter(|&x| x == '(').count() - line.chars().filter(|&x| x == ')').count()
}
fn get_result_2(file_content: &Vec<String>) -> Option<usize> {
    let mut floor = 0;
    let line = &file_content[0];
    for (i, ch) in line.chars().enumerate() {
        if ch == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor < 0 {
            return Some(i + 1);
        }
    }
    None
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 0, None),
        ("test2.txt", 3, Some(1)),
        ("input.txt", 232, Some(1783)),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(file_content.len(), 1);
        let line = &file_content[0];
        assert!(line.chars().all(|x| x == '(' || x == ')'));

        assert_eq!(get_result_1(&file_content), result_1);

        assert_eq!(get_result_2(&file_content), result_2);
    }
    Ok(())
}
