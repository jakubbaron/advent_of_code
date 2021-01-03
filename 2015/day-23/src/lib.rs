use std::collections::HashMap;

pub trait Execute {
    fn execute(&self, registers: &mut HashMap<char, usize>, current_id: usize) -> usize;
}

#[derive(Debug)]
pub enum Instruction {
    HLF(Half),
    TPL(Tripple),
    INC(Increment),
    JMP(Jump),
    JIE(JumpIfEven),
    JIO(JumpIfOne),
}

#[derive(Debug)]
pub struct Half {
    register: char,
}

impl Execute for Half {
    fn execute(&self, registers: &mut HashMap<char, usize>, current_id: usize) -> usize {
        println!("{}: Executing Half on {}", current_id, self.register);
        registers.entry(self.register).and_modify(|x| *x /= 2);
        current_id + 1
    }
}

impl Half {
    pub fn new(register: char) -> Half {
        Half { register }
    }
}

#[derive(Debug)]
pub struct Tripple {
    register: char,
}

impl Execute for Tripple {
    fn execute(&self, registers: &mut HashMap<char, usize>, current_id: usize) -> usize {
        println!("{}: Executing Tripple on {}", current_id, self.register);
        registers.entry(self.register).and_modify(|x| *x *= 3);
        current_id + 1
    }
}

impl Tripple {
    pub fn new(register: char) -> Tripple {
        Tripple { register }
    }
}

#[derive(Debug)]
pub struct Increment {
    register: char,
}

impl Execute for Increment {
    fn execute(&self, registers: &mut HashMap<char, usize>, current_id: usize) -> usize {
        println!("{}: Increment on {}", current_id, self.register);
        registers.entry(self.register).and_modify(|x| *x += 1);
        current_id + 1
    }
}

impl Increment {
    pub fn new(register: char) -> Increment {
        Increment { register }
    }
}

#[derive(Debug)]
pub struct Jump {
    jump: i64,
}

impl Execute for Jump {
    fn execute(&self, _registers: &mut HashMap<char, usize>, current_id: usize) -> usize {
        println!("{}: Jump by {}", current_id, self.jump);
        (current_id as i64 + self.jump) as usize
    }
}

impl Jump {
    pub fn new(jump: i64) -> Jump {
        Jump { jump }
    }
}

#[derive(Debug)]
pub struct JumpIfEven {
    register: char,
    jump: i64,
}

impl Execute for JumpIfEven {
    fn execute(&self, registers: &mut HashMap<char, usize>, current_id: usize) -> usize {
        println!(
            "{}: JumpIfEven on {} by {} {:?}",
            current_id, self.register, self.jump, registers
        );
        match registers.get(&self.register).unwrap() % 2 == 0 {
            true => (current_id as i64 + self.jump) as usize,
            false => current_id + 1,
        }
    }
}

impl JumpIfEven {
    pub fn new(register: char, jump: i64) -> JumpIfEven {
        JumpIfEven { register, jump }
    }
}

#[derive(Debug)]
pub struct JumpIfOne {
    register: char,
    jump: i64,
}

impl Execute for JumpIfOne {
    fn execute(&self, registers: &mut HashMap<char, usize>, current_id: usize) -> usize {
        println!(
            "{}: JumpIfOne on {} by {} {:?}",
            current_id, self.register, self.jump, registers
        );
        match registers.get(&self.register).unwrap() == &1 {
            true => (current_id as i64 + self.jump) as usize,
            false => current_id + 1,
        }
    }
}
impl JumpIfOne {
    pub fn new(register: char, jump: i64) -> JumpIfOne {
        JumpIfOne { register, jump }
    }
}

pub fn execute_instructions(
    instructions: &Vec<Instruction>,
    registers: &mut HashMap<char, usize>,
) -> usize {
    let mut i = 0;
    while i < instructions.len() {
        i = match &instructions[i] {
            Instruction::HLF(s) => s.execute(registers, i),
            Instruction::TPL(s) => s.execute(registers, i),
            Instruction::INC(s) => s.execute(registers, i),
            Instruction::JMP(s) => s.execute(registers, i),
            Instruction::JIE(s) => s.execute(registers, i),
            Instruction::JIO(s) => s.execute(registers, i),
        }
    }
    println!("{:?}", registers);
    *registers.get(&'b').unwrap()
}

pub fn parse_instructions(file_content: &Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
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
    instructions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        assert!(false);
    }
}
