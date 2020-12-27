use std::io::{self};

fn get_result_1(file_content: &Vec<String>) -> usize {
    let mut total_len = 0;
    let mut total_mem_len = 0;
    for line in file_content.iter() {
        total_mem_len += line.len();
        let new_line = line
            .replace("\\\\", "#")
            .replace("\\\"", "#")
            .replace("\"", "");
        let len_modifier = new_line.matches("\\x").count() * 3; // from 4 chars -> 1 char
        total_len += (new_line.len() - len_modifier);
    }
    total_mem_len - total_len
}

fn get_result_2(file_content: &Vec<String>) -> usize {
    let mut total_len = 0;
    let mut super_len = 0;
    for line in file_content.iter() {
        total_len += line.len();
        super_len += line.len() + line.matches("\\").count() + line.matches("\"").count() + 2 /* two quotes */;
    }
    super_len - total_len
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 12, 19), ("input.txt", 1371, 1)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(get_result_1(&file_content), result_1);
        assert_eq!(get_result_2(&file_content), result_2);
    }
    Ok(())
}
