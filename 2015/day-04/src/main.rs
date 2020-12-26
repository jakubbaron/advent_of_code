use std::io::{self};

fn md5_starts_with(line: &str, starts_with: &str) -> usize {
    let mut number = 1;
    loop {
        let digest = md5::compute(format!("{}{}", line, number));
        let md5_str = format!("{:x}", digest);
        if md5_str.starts_with(starts_with) {
            break;
        } else {
            number += 1;
        }
    }
    number
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 609043, 6742839),
        ("test2.txt", 1048970, 5714438),
        ("input.txt", 346386, 9958218),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(file_content.len(), 1);
        let line = &file_content[0];
        assert_eq!(md5_starts_with(&line, "00000"), result_1);
        assert_eq!(md5_starts_with(&line, "000000"), result_2);
    }
    Ok(())
}
