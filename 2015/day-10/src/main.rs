use std::io::{self};

fn play_game(line: &str, iterations: usize) -> usize {
    let mut chars: Vec<char> = line.chars().collect();
    let mut new_chars: String = "".to_owned();
    for _ in 0..iterations {
        let mut count = 1;
        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                count += 1;
            } else {
                new_chars.push_str(&count.to_string());
                new_chars.push_str(&chars[i - 1].to_string());
                count = 1;
            }
        }
        new_chars.push_str(&count.to_string());
        new_chars.push_str(&chars[chars.len() - 1].to_string());
        chars = new_chars.chars().collect();
        new_chars.clear();
    }
    chars.len()
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 237746, 3369156),
        ("input.txt", 252594, 3579328),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let line = &file_content[0];
        assert_eq!(play_game(&line, 40), result_1);
        assert_eq!(play_game(&line, 50), result_2);
    }
    Ok(())
}
