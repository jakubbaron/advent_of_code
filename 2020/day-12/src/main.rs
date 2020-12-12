use regex::Regex;
use std::io::{self};

#[derive(Debug, Clone)]
struct Instruction {
    letter: String,
    number: i32,
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn abs(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn rotate(&mut self, angle: f32) {
        let s = angle.to_radians().sin().round();
        let c = angle.to_radians().cos().round();
        let ynew = self.y as f32 * c - self.x as f32 * s;
        let xnew = self.y as f32 * s + self.x as f32 * c;
        self.x = xnew as i32;
        self.y = ynew as i32;
    }
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
    let mut pos = Point { x: 0, y: 0 };
    for instruction in &instructions {
        println!("{:?}", instruction);
        let Instruction { letter, number } = instruction;
        let number = *number;
        let helper = match letter.as_str() {
            "F" => directions[dir_idx as usize].to_string(),
            _ => letter.to_string(),
        };

        let point = match helper.as_str() {
            "N" => Point { y: number, x: 0 },
            "S" => Point { y: -number, x: 0 },
            "E" => Point { y: 0, x: number },
            "W" => Point { y: 0, x: -number },
            _ => Point { x: 0, y: 0 },
        };
        pos.add(&point);

        let spin_move = match helper.as_str() {
            "L" => -number / 90,
            "R" => number / 90,
            _ => 0,
        };
        dir_idx += spin_move % 4;
        dir_idx = modulo(dir_idx, 4);
        println!("Pos {:?} direction {}", pos, directions[dir_idx as usize]);
    }
    println!("Manhatann distance {}", pos.abs());
    assert_eq!(pos.abs(), 441);

    let mut waypoint = Point { x: 10, y: 1 };
    let mut ship_pos = Point { x: 0, y: 0 };
    for instruction in &instructions {
        println!("{:?}", instruction);
        let Instruction { letter, number } = instruction;
        let number = *number;
        let waypoint_move = match letter.as_str() {
            "N" => Point { y: number, x: 0 },
            "S" => Point { y: -number, x: 0 },
            "E" => Point { y: 0, x: number },
            "W" => Point { y: 0, x: -number },
            _ => Point { x: 0, y: 0 },
        };
        let angle: f32 = match letter.as_str() {
            "L" => -number,
            "R" => number,
            _ => 0,
        } as f32;
        let ship_move = match letter.as_str() {
            "F" => Point {
                y: number * waypoint.y,
                x: number * waypoint.x,
            },
            _ => Point { x: 0, y: 0 },
        };
        waypoint.rotate(angle);
        waypoint.add(&waypoint_move);
        ship_pos.add(&ship_move);
        println!("ShipPos {:?} Waypoint {:?}", ship_pos, waypoint);
    }
    println!("Manhatann distance {}", ship_pos.abs());
    assert_eq!(ship_pos.abs(), 40014);
    Ok(())
}
