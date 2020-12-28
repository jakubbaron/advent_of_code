use std::collections::HashSet;
use std::io::{self};

fn add1_char(c: char) -> char {
    std::char::from_u32(c as u32 + 1).unwrap_or(c)
}

fn is_valid_password(chars: &Vec<char>) -> bool {
    let not_allowed: HashSet<char> = vec!['i', 'l', 'o'].into_iter().collect();
    if !chars.iter().all(|x| !not_allowed.contains(x)) {
        return false;
    }
    let mut has_two_pairs = false;
    'outer: for i in 1..chars.len() - 2 {
        if chars[i] == chars[i - 1] {
            for j in (i + 1)..chars.len() {
                if chars[j] == chars[i] {
                    break;
                }
                if chars[j] == chars[j - 1] {
                    has_two_pairs = true;
                    break 'outer;
                }
            }
        }
    }
    if !has_two_pairs {
        return false;
    }
    let mut has_consecutive_incremental_chars = false;
    for i in 0..chars.len() - 2 {
        if chars[i] > 'x' {
            continue;
        }

        let add_1 = add1_char(chars[i]);
        let add_2 = add1_char(add_1);
        if chars[i + 1] == add_1 && chars[i + 2] == add_2 {
            has_consecutive_incremental_chars = true;
            break;
        }
    }
    if !has_consecutive_incremental_chars {
        return false;
    }

    return true;
}

fn increment_password(chars: &mut Vec<char>) {
    for i in (0..chars.len()).rev() {
        let add_1 = add1_char(chars[i]);
        if add_1 <= 'z' {
            chars[i] = add_1;
            break;
        } else {
            chars[i] = 'a';
        }
    }
}

fn main() -> io::Result<()> {
    let files_results = vec![
        // ("test.txt", "ghjaabcc", 1),
        ("test2.txt", "abcdffaa", "abcdffbb"),
        ("input.txt", "vzbxxyzz", "vzcaabcc"),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let line = &file_content[0];
        let mut pass_chars: Vec<char> = line.chars().collect();
        increment_password(&mut pass_chars);
        while !is_valid_password(&pass_chars) {
            increment_password(&mut pass_chars);
        }
        let password = pass_chars.iter().collect::<String>();
        println!("End password: `{}`", password);
        assert_eq!(password, result_1);

        increment_password(&mut pass_chars);
        while !is_valid_password(&pass_chars) {
            increment_password(&mut pass_chars);
        }
        let password = pass_chars.into_iter().collect::<String>();
        assert_eq!(password, result_2);
    }
    Ok(())
}
