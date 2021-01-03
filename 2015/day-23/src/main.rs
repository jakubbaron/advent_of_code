use day_23::{parse_instructions, execute_instructions, Execute, Half, Increment, Instruction, Jump, JumpIfEven, JumpIfOne, Tripple};
use std::collections::HashMap;
use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 0, 0),
        ("input.txt", 170, 247)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut registers: HashMap<char, usize> = vec![('a', 0), ('b', 0)].into_iter().collect();
        let instructions = parse_instructions(&file_content);
        assert_eq!(execute_instructions(&instructions, &mut registers), result_1);

        let mut registers: HashMap<char, usize> = vec![('a', 1), ('b', 0)].into_iter().collect();
        assert_eq!(execute_instructions(&instructions, &mut registers), result_2);
    }
    Ok(())
}
