// use std::collections::HashMap;
// use std::collections::HashSet;
use regex::Regex;
use std::io::{self};

#[derive(Debug, Clone)]
struct Instruction {
    letter: String,
    number: i32,
}

fn modulo(x: i32, m: i32) -> i32 {
    (x % m + m) % m
}

fn main() -> io::Result<()> {
    let f = "test.txt";
    let f = "input.txt";

    let vec: Vec<String> = std::fs::read_to_string(f)?
        .lines()
        .map(|x| x.to_string())
        .collect();
    let re = Regex::new(r"^([WNSEWLRF])(\d+)$").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in vec.into_iter() {
        if !re.is_match(&line) {
            println!("NOPE {}", &line);
        }
        let caps = re.captures(&line).unwrap();
        let letter = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let number = caps
            .get(2)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        instructions.push(Instruction { letter, number });
    }
    let instructions: Vec<Instruction> = instructions;
    let directions = vec!["N", "E", "S", "W"];
    let mut dir_idx = 1_i32;
    let mut pos = (0_i32, 0_i32);
    for instruction in &instructions {
        println!("{:?}", instruction);
        let Instruction { letter, number } = instruction;
        let number = *number;
        let mut vertical_move = 0;
        let mut horizontal_move = 0;
        let mut spin_move = 0;
        let helper: String;
        if letter == "F" {
            helper = directions[dir_idx as usize].to_string();
        } else {
            helper = letter.to_string();
        }
        match helper.as_str() {
            "N" => {
                vertical_move = number;
            }
            "S" => {
                vertical_move = -number;
            }
            "E" => {
                horizontal_move = number;
            }
            "W" => {
                horizontal_move = -number;
            }
            "L" => {
                spin_move = -number / 90;
            }
            "R" => {
                spin_move = number / 90;
            }
            _ => (),
        }
        pos.0 += vertical_move;
        pos.1 += horizontal_move;
        dir_idx += spin_move % 4;
        dir_idx = modulo(dir_idx, 4);
        println!("Pos {:?} direction {}", pos, directions[dir_idx as usize]);
    }
    println!("Manhatann distance {}", pos.0.abs() + pos.1.abs());

    let mut waypoint = (1_i32, 10_i32);
    let mut ship_pos = (0_i32, 0_i32);
    for instruction in &instructions {
        println!("{:?}", instruction);
        let Instruction { letter, number } = instruction;
        let number = *number;
        let mut w_vertical_move = 0;
        let mut w_horizontal_move = 0;
        let mut s_vertical_move = 0;
        let mut s_horizontal_move = 0;
        let mut angle = 0_f32;
        match letter.as_str() {
            "N" => {
                w_vertical_move = number;
            }
            "S" => {
                w_vertical_move = -number;
            }
            "E" => {
                w_horizontal_move = number;
            }
            "W" => {
                w_horizontal_move = -number;
            }
            "L" => {
                angle = -number as f32;
            }
            "R" => {
                angle = number as f32;
            }
            "F" => {
                s_vertical_move = number * waypoint.0;
                s_horizontal_move = number * waypoint.1;
            }
            _ => (),
        }
        let s = angle.to_radians().sin().round();
        let c = angle.to_radians().cos().round();
        let xnew = waypoint.0 as f32 * c - waypoint.1 as f32 * s;
        let ynew = waypoint.0 as f32 * s + waypoint.1 as f32 * c;
        waypoint = (xnew as i32, ynew as i32);
        waypoint.0 += w_vertical_move;
        waypoint.1 += w_horizontal_move;
        ship_pos.0 += s_vertical_move;
        ship_pos.1 += s_horizontal_move;
        println!("ShipPos {:?} waypoint {:?}", ship_pos, waypoint);
    }
    println!("Manhatann distance {}", ship_pos.0.abs() + ship_pos.1.abs());
    Ok(())
}
