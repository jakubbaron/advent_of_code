use std::io::{self};

fn is_nice(line: &str) -> bool {
    let vowels = vec!["a", "e", "i", "o", "u"];
    if line.len() < 3 {
        return false;
    }
    let mut vowel_count = 0;
    for vowel in vowels.iter() {
        vowel_count += line.matches(vowel).count();
    }
    if vowel_count < 3 {
        return false;
    }

    let chars: Vec<char> = line.chars().collect();
    let (mut previous, chars) = (&chars[0], &chars[1..]);
    let mut has_doubles = false;
    for ch in chars.iter() {
        if previous == ch {
            has_doubles = true;
            break;
        }
        previous = ch;
    }
    if !has_doubles {
        return false;
    }
    let not_alloweds = vec!["ab", "cd", "pq", "xy"];
    for not_allowed in not_alloweds.iter() {
        if line.contains(not_allowed) {
            return false;
        }
    }
    true
}

fn is_nice_2(line: &str) -> bool {
    let mut has_pair = false;
    for i in 1..line.len() - 1 {
        let current_part = &line[i - 1..=i];
        if line[i + 1..].contains(&current_part) {
            has_pair = true;
            break;
        }
    }
    if !has_pair {
        return false;
    }
    let chars: Vec<char> = line.chars().collect();
    let mut has_double_letters = false;
    for i in 2..line.len() {
        if chars[i - 2] == chars[i] {
            has_double_letters = true;
            break;
        }
    }
    if !has_double_letters {
        return false;
    }

    true
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 2, 0),
        ("test2.txt", 0, 2),
        ("input.txt", 255, 1),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(
            file_content
                .iter()
                .map(|line| is_nice(line))
                .filter(|&x| x)
                .count(),
            result_1
        );
        assert_eq!(
            file_content
                .iter()
                .map(|line| is_nice_2(line))
                .filter(|&x| x)
                .count(),
            result_2
        );
    }
    Ok(())
}
