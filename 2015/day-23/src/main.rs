use day_23::{Execute, Half, Increment, Instruction, Jump, JumpIfEven, JumpIfOne, Tripple};
use std::collections::HashMap;
use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 0, 1), ("input.txt", 170, 1)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut registers: HashMap<char, usize> = vec![('a', 0), ('b', 0)].into_iter().collect();
        for line in file_content.iter() {
            let splitted: Vec<&str> = line.split(" ").collect();
            let register = splitted[1].replace(",", "").chars().collect::<Vec<char>>()[0];

            instructions.push(match splitted[0] {
                "hlf" => Instruction::HLF(Half::new(register)),
                "tpl" => Instruction::TPL(Tripple::new(register)),
                "inc" => Instruction::INC(Increment::new(register)),
                "jmp" => {
                    let parsed = match splitted[1].parse::<i64>() {
                        Ok(val) => val,
                        Err(e) => panic!("Wrong instruction for jmp {}", e),
                    };
                    Instruction::JMP(Jump::new(parsed))
                }
                "jie" => {
                    let parsed = match splitted[2].parse::<i64>() {
                        Ok(val) => val,
                        Err(e) => panic!("Wrong instruction for jie {}", e),
                    };
                    Instruction::JIE(JumpIfEven::new(register, parsed))
                }
                "jio" => {
                    let parsed = match splitted[2].parse::<i64>() {
                        Ok(val) => val,
                        Err(e) => panic!("Wrong instruction for jio {}", e),
                    };
                    Instruction::JIO(JumpIfOne::new(register, parsed))
                }
                _ => {
                    panic!("Unexpected instruction {}", splitted[0]);
                }
            });
        }
        for instr in instructions.iter() {
            println!("Instruction: {:?}", instr);
        }

        let mut i = 0;
        while i < instructions.len() {
            i = match &instructions[i] {
                Instruction::HLF(s) => s.execute(&mut registers, i),
                Instruction::TPL(s) => s.execute(&mut registers, i),
                Instruction::INC(s) => s.execute(&mut registers, i),
                Instruction::JMP(s) => s.execute(&mut registers, i),
                Instruction::JIE(s) => s.execute(&mut registers, i),
                Instruction::JIO(s) => s.execute(&mut registers, i),
            }
        }
        println!("{:?}", registers);
        assert_eq!(*registers.get(&'b').unwrap(), result_1);
    }
    Ok(())
}
