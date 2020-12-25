use std::io::{self};
use std::collections::HashMap;
use regex::Regex;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Color {
    White,
    Black,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point{x, y}
    }
    fn add_mut(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
    fn add(&self, other: &Point) -> Point {
        Point{x: self.x + other.x, y: self.y + other.y}
    }
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 10, 2208),
        ("test2.txt", 1, 0),
        ("test3.txt", 1, 0),
        ("input.txt", 330, 3711)
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        let direction_map: HashMap<&str, Point> = vec![
            ("e", Point::new(2, 0)),
            ("se", Point::new(1, -1)),
            ("sw", Point::new(-1, -1)),
            ("w", Point::new(-2, 0)),
            ("nw", Point::new(-1, 1)),
            ("ne", Point::new(1, 1)),
        ].into_iter().collect();
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let re = Regex::new(r"(e|se|sw|w|nw|ne)+?").unwrap();
        let mut tiles: HashMap<Point, Color> = vec![(Point::new(0,0), Color::White)].into_iter().collect();
        for line in file_content.iter() {
            let mut instructions: Vec<Point> = Vec::new();
            for cap in re.captures_iter(line) {
                // println!("{}", &cap[1]);
                instructions.push(direction_map.get(&cap[1]).unwrap().clone());
            }
            let mut address = Point::new(0,0);
            for instr in instructions.into_iter() {
                // println!("Instr {:?}", instr);
                address.add_mut(&instr);
                tiles.entry(address).or_insert(Color::White);
            }
            // println!("End Address: {:?}", address);
            tiles.entry(address).and_modify(|x| {*x = match x  {
                Color::White => Color::Black,
                Color::Black => Color::White,
            }}).or_insert(Color::Black);
        }
        assert_eq!(tiles.iter().filter(|(_, v)| **v == Color::Black).count(), result_1);

        let mut previous_tiles = tiles.clone();
        tiles.clear();
        let neighbours: Vec<Point> = direction_map.values().cloned().collect();
        for _ in 0..100 {
            for (point, color) in previous_tiles.iter() {
                let mut black_count = 0;
                for n in neighbours.iter() {
                    match previous_tiles.get(&point.add(n)) {
                        Some(c) => {
                            match *c {
                                Color::Black => { black_count += 1;},
                                _ => (),
                            }
                        },
                        None => (),
                    }
                }
                let new_color = match *color {
                    Color::White => {
                        if black_count == 2 {
                            Color::Black
                        } else {
                            Color::White
                        }
                    },
                    Color::Black => {
                        if black_count == 0 || black_count > 2 {
                            Color::White
                        } else {
                            Color::Black
                        }
                    },
                };
                // println!("Point {:?} old color {:?} new color {:?} black_count {} flipping {}", point, color, new_color, black_count, new_color != *color);
                tiles.insert(point.clone(), new_color);
                if *color == Color::Black {
                    for n in neighbours.iter() {
                        let tmp_point: Point = point.add(n);
                        if previous_tiles.contains_key(&tmp_point) {
                            continue;
                        }
                        // println!("TRYING NEW POINT {:?} from {:?}", tmp_point, point);
                        let mut black_count = 0;
                        for nn in neighbours.iter() {
                            match previous_tiles.get(&tmp_point.add(nn)) {
                                Some(c) => {
                                    match c{
                                        Color::Black => { black_count += 1;},
                                        _ => (),
                                    }
                                },
                                None => (),
                            }
                        }
                        let new_color = if black_count == 2 {
                            Color::Black
                        } else {
                            Color::White
                        };
                        if new_color == Color::Black {
                            // println!("New Point {:?} color: {:?} black_count {}", tmp_point, new_color, black_count);
                            tiles.insert(tmp_point, new_color);
                        }
                    }
                }
            }
            previous_tiles = tiles.clone();
        }
        assert_eq!(tiles.iter().filter(|(_, v)| **v == Color::Black).count(), result_2);
    }
    Ok(())
}
