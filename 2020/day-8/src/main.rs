use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

#[derive(Debug)]
struct Instruction {
    instr: String,
    number: i32,
}

fn get_instruction(line: &String) -> Instruction {
    let tmp: Vec<&str> = line.split(" ").collect();
    let instr: String = tmp[0].trim().to_string();
    let number = tmp[1].parse::<i32>().unwrap();
    Instruction { instr, number }
}
fn run_code(
    vec: Vec<String>,
    mut id: i32,
    mut acc: i32,
    mut visited: HashSet<i32>,
    is_branched: bool,
) -> Result<i32, i32> {
    while id != vec.len() as i32 {
        visited.insert(id.clone());
        let instr = get_instruction(&vec[id as usize]);
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
                vec_cp[id as usize] = format!("jmp {}", instr.number).to_string();
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
                vec_cp[id as usize] = format!("nop {}", instr.number).to_string();
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
    let f = File::open("test.txt")?;
    // let f = File::open("test2.txt")?;
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut vec = Vec::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        vec.push(my_string);
    }

    let id: i32 = 0;
    let acc = 0;
    let visited: HashSet<i32> = HashSet::new();
    match run_code(vec, id, acc, visited, false) {
        Ok(var) => println!("Result value {}", var),
        Err(var) => println!("Err {}", var),
    }

    Ok(())
}
