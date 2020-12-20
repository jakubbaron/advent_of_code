use std::io::{self};
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;

#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Debug)]
enum Side {
    Bottom,
    Top,
    Left,
    Right,
}

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Debug)]
struct Frame {
    title: String,
    frame_no: usize,
    data: Vec<Vec<char>>,
    sides: HashMap<Side, Vec<char>>,
    neighbours: HashSet<Side>,
}

impl Frame {
    fn new(vec: &Vec<Vec<char>>) -> Frame {
        assert!(vec.len() > 2);
        assert!(vec[0].len() > 2);
        let title: String = vec[0].iter().collect();
        let tmp:Vec<&str> = title.split(" ").collect();
        let frame_no = tmp[1].replace(":", " ").trim().parse::<usize>().unwrap();
        let data = vec[1..vec.len()].to_vec();

        let bottom = data[data.len()-1].to_vec();
        let top = data[0].to_vec();
        let left:Vec<char> = data.iter().map(|row| row[0]).collect();
        let right :Vec<char> = data.iter().map(|row| row[data.len() - 1]).collect();
        let sides: HashMap<Side, Vec<char>> = vec![
            (Side::Bottom, bottom),
            (Side::Top, top),
            (Side::Left, left),
            (Side::Right, right),
        ].into_iter().collect();
        Frame {
            title,
            frame_no,
            data,
            sides,
            neighbours: HashSet::new(),
        }
    }
    fn try_neighbour(&mut self, mut neighbour: &mut Frame) -> bool {
        if self.neighbours.len() == 4 || neighbour.neighbours.len() == 4 {
            return false;
        }
        let sides = vec![Side::Left, Side::Right, Side::Bottom, Side::Top];
        for side in sides.iter() {
            if self.neighbours.contains(side) {
                continue;
            }
            for match_side in sides.iter() {
                if neighbour.neighbours.contains(match_side) {
                    continue;
                }
                let their_side = neighbour.sides.get(match_side).unwrap();
                let frame_side = self.sides.get(side).unwrap();
                // println!("Side 1 {:?}",frame_side);
                // println!("side {:?} match_side {:?}", side, match_side);
                // println!("Side 2 {:?}",their_side);
                if frame_side == their_side {
                    println!("Matching {} with {}", self.frame_no, neighbour.frame_no);
                    println!("{:?} {:?}", frame_side, their_side);
                    self.add_neighbour(side);
                    neighbour.add_neighbour(match_side);
                    return true;
                }
                let reverse:Vec<char> = their_side.to_vec().into_iter().rev().collect();
                if frame_side == &reverse {
                    println!("Matching {} with {}", self.frame_no, neighbour.frame_no);
                    println!("{:?} {:?}", frame_side, their_side);
                    self.add_neighbour(side);
                    neighbour.add_neighbour(match_side);
                    return true;
                }
            }
        }
        return false
    }
    fn add_neighbour(&mut self, side: &Side) {
        self.neighbours.insert(side.clone());
    }
}
fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 20899048083289_usize, 1),
        ("input.txt", 20913499394191, 1),

    ];
    for (f, result_1, result_2) in files_results.iter() {
        println!("{}", f);
        let file_content: Vec<Vec<char>> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.chars().collect())
            .collect();
        let mut tmp: Vec<Vec<char>> = Vec::new();
        let mut frames: HashMap<usize, RefCell<Frame>> = HashMap::new();
        for line in file_content.iter() {
            if line.is_empty() {
                let frame = Frame::new(&tmp);
                frames.insert(frame.frame_no, RefCell::new(frame));
                tmp.clear();
            }
            else {
                tmp.push(line.to_vec());
            }
        }
        let keys: Vec<usize> = frames.keys().cloned().collect();
        for i in 0..keys.len() {
            let key_1 = &keys[i];
            let mut frame_1 = frames.get(key_1).unwrap().borrow_mut();
            for j in 0..keys.len() {
                if i == j {
                    continue;
                }
                let key_2 = &keys[j];
                let mut frame_2 = frames.get(key_2).unwrap().borrow_mut();
                // println!("Trying to match {} {}", frame_1.frame_no, frame_2.frame_no);
                frame_1.try_neighbour(&mut frame_2);
            }
        }
        let mut res_1 = 1;
        for frame in frames.values() {
            println!("{} {:?}", frame.borrow().frame_no, frame.borrow().neighbours);
            if frame.borrow().neighbours.len() == 2 {
                res_1*=frame.borrow().frame_no;
            }
        }
        assert_eq!(res_1, *result_1);
    }
    Ok(())
}
