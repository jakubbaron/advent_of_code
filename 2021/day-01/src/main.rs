use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 7, 5),
        (
            "input.txt",
            1288,
            1311,
        ),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<u32> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect();
        let mut res_1 = 0;
        for i in 1..file_content.len() {
            if file_content[i] > file_content[i-1] {
                res_1 += 1;
            }
        }
        assert_eq!(res_1, result_1);

        let mut res_2 = 0;
        let mut helper: Vec<u32> = vec![0; file_content.len()];

        for i in 2..file_content.len() {
            for j in 0..3 {
                helper[i] += file_content[i - j];
            }
        }
        for i in 3..helper.len() {
            if helper[i] > helper[i-1] {
                res_2 += 1;
            }
        }

        assert_eq!(res_2, result_2);
    }
    Ok(())
}
