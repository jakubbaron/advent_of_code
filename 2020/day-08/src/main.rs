use std::collections::HashSet;
use std::io::{self};

#[derive(Debug)]
#[derive(Clone)]
struct Instruction {
    instr: String,
    number: i32,
}

fn get_instruction(line: String) -> Instruction {
    let tmp: Vec<&str> = line.split(" ").collect();
    let instr: String = tmp[0].trim().to_string();
    let number = tmp[1].parse::<i32>().unwrap();
    Instruction { instr, number }
}
fn run_code(
    vec: Vec<Instruction>,
    mut id: i32,
    mut acc: i32,
    mut visited: HashSet<i32>,
    is_branched: bool,
) -> Result<i32, i32> {
    while id != vec.len() as i32 {
        visited.insert(id.clone());
        let instr = &vec[id as usize];
        let new_id;
        let mut new_acc = acc;

        match instr.instr.as_str() {
            "nop" => {
                new_id = id + 1;
            }
            "acc" => {
                new_acc = acc + instr.number;
                new_id = id + 1;
            }
            "jmp" => {
                new_id = id + instr.number;
            }
            _ => {
                return Err(-123123123);
            }
        }
        if visited.contains(&new_id) {
            println!("Visiting the same id {} {}", new_id, acc);
            return Err(acc);
        }

        if !is_branched {
            if instr.instr.as_str() == "nop" {
                let mut vec_cp = vec.to_vec();
                vec_cp[id as usize] = Instruction{instr: "jmp".to_string(), number: instr.number};
                match run_code(
                    vec_cp,
                    id + instr.number,
                    acc.clone(),
                    visited.clone(),
                    true,
                ) {
                    Ok(val) => return Ok(val),
                    Err(_) => (),
                }
            }
            if instr.instr.as_str() == "jmp" {
                let mut vec_cp = vec.to_vec();
                vec_cp[id as usize] = Instruction{instr: "nop".to_string(), number: instr.number};
                match run_code(vec_cp, id + 1, acc.clone(), visited.clone(), true) {
                    Ok(val) => return Ok(val),
                    Err(_) => (),
                }
            }
        }

        id = new_id;
        acc = new_acc;
    }
    return Ok(acc);
}

fn main() -> io::Result<()> {
    let f = "test.txt";
    // let f = File::open("test2.txt")?;
    let f = "input.txt";
    let vec: Vec<String> = std::fs::read_to_string(f)?
        .lines()
        .map(|x| x.to_string())
        .collect();

    let id: i32 = 0;
    let acc = 0;
    let visited: HashSet<i32> = HashSet::new();
    let instructions: Vec<Instruction> = vec.into_iter().map(|x| get_instruction(x)).collect();
    match run_code(instructions, id, acc, visited, false) {
        Ok(var) => println!("Result value {}", var),
        Err(var) => println!("Err {}", var),
    }

    Ok(())
}
