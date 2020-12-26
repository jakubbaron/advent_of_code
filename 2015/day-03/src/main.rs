use std::collections::{HashMap, HashSet};
use std::io::{self};

#[derive(Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn get_result_1(file_content: &Vec<String>) -> usize {
    let directions: HashMap<char, Point> = vec![
        ('>', Point::new(1, 0)),
        ('<', Point::new(-1, 0)),
        ('^', Point::new(0, 1)),
        ('v', Point::new(0, -1)),
    ]
    .into_iter()
    .collect();

    let mut houses: HashSet<Point> = vec![Point::new(0, 0)].into_iter().collect();
    for line in file_content.iter() {
        let mut pt = Point::new(0, 0);
        for ch in line.chars() {
            pt = pt.add(directions.get(&ch).unwrap());
            houses.insert(pt.clone());
        }
    }
    houses.len()
}

fn get_result_2(file_content: &Vec<String>) -> usize {
    let directions: HashMap<char, Point> = vec![
        ('>', Point::new(1, 0)),
        ('<', Point::new(-1, 0)),
        ('^', Point::new(0, 1)),
        ('v', Point::new(0, -1)),
    ]
    .into_iter()
    .collect();

    let mut santa_houses: HashSet<Point> = vec![Point::new(0, 0)].into_iter().collect();
    let mut robot_houses: HashSet<Point> = vec![Point::new(0, 0)].into_iter().collect();
    for line in file_content.iter() {
        let mut santa_pt = Point::new(0, 0);
        let mut robot_pt = Point::new(0, 0);
        for (i, ch) in line.chars().enumerate() {
            if i % 2 == 0 {
                santa_pt = santa_pt.add(directions.get(&ch).unwrap());
                santa_houses.insert(santa_pt.clone());
            } else {
                robot_pt = robot_pt.add(directions.get(&ch).unwrap());
                robot_houses.insert(robot_pt.clone());
            }
        }
    }
    santa_houses.union(&robot_houses).count()
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 4, 3),
        ("test2.txt", 2, 11),
        ("input.txt", 2081, 2341),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(get_result_1(&file_content), result_1);
        assert_eq!(get_result_2(&file_content), result_2);
    }
    Ok(())
}
