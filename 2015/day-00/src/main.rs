use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 1, 1),
        ("input.txt", 1, 1)
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        for line in file_content.iter() {

        }
    }
    Ok(())
}
