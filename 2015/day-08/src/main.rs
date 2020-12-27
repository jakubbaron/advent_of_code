use std::io::{self};
// use regex::Regex;

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 12, 1),
        ("input.txt", 1387, 1)
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut total_len = 0;
        let mut total_mem_len = 0;
        for line in file_content.iter() {
            total_mem_len += line.len();
            let mut len_modifier = 2;
            len_modifier += line.matches("\\\\").count();
            len_modifier += line.matches("\\x").count() * 3; // from 4 chars -> 1 char
            len_modifier += line.matches("\\\"").count();
            total_len += (line.len() - len_modifier);
            println!(
                "Line {}, line.len() {}, len_modfier: {}, subtracted: {}",
                line,
                line.len(),
                len_modifier,
                line.len() - len_modifier,
            );
        }
        let res_1 = total_mem_len - total_len;
        assert_eq!(res_1, result_1);
    }
    Ok(())
}
