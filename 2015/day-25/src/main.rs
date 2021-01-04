use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![("input.txt", 2650453)];
    for (f, result_1) in files_results.into_iter() {
        println!("File: {}", f);
        let sought_row = 2978;
        let sought_col = 3083;
        let mut last_val = 20151125_usize;
        let mut diagonal = 2;
        'outer: loop {
            for i in 0..diagonal {
                let r = diagonal - i;
                let c = i + 1;
                last_val = last_val * 252533 % 33554393;
                if r == sought_row && c == sought_col {
                    break 'outer;
                }
            }
            diagonal += 1;
        }
        println!("Code from the machine: {}", last_val);
        assert_eq!(last_val, result_1);
    }
    Ok(())
}
