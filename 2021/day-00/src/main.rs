use std::io::{self};

fn part_1(file_content: &Vec<String>) -> i32 {
    0
}

fn part_2(file_content: &Vec<String>) -> i32 {
    0
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 0, 0), ("input.txt", 0, 0)];
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
