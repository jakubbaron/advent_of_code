use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![
        ("input.txt", 2650453)
    ];
    for (f, result_1) in files_results.into_iter() {
        println!("File: {}", f);
        let sought_row = 2978;
        let sought_col = 3083;
        let max_of_both = std::cmp::max(sought_row, sought_col) * 2;
        let mut results = vec![vec![0_usize; max_of_both]; max_of_both];
        let mut last_val = 20151125_usize;
        let mut diagonal = 2;
        results[1][1] = last_val;
        while results[sought_row][sought_col] == 0 {
            for i in 0..diagonal {
                let r = diagonal - i;
                let c = i + 1;
                results[r][c] = last_val * 252533 % 33554393;
                last_val = results[r][c];
            }
            diagonal += 1;
        }
        println!("Code from the machine: {}", results[sought_row][sought_col]);
        assert_eq!(results[sought_row][sought_col], result_1);
    }
    Ok(())
}
