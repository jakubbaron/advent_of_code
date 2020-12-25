use regex::Regex;
use std::collections::HashMap;
use std::io::{self};

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
        Point { x, y }
    }
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn get_neighbouring_black_tiles(
    point: &Point,
    previous_tiles: &HashMap<Point, Color>,
    neighbours: &Vec<Point>,
) -> usize {
    let mut black_count = 0;
    for n in neighbours.iter() {
        match previous_tiles.get(&point.add(n)) {
            Some(c) => match *c {
                Color::Black => {
                    black_count += 1;
                }
                _ => (),
            },
            None => (),
        }
    }
    black_count
}

fn get_new_color(black_count: usize, color: Color) -> Color {
    match color {
        Color::White => {
            if black_count == 2 {
                Color::Black
            } else {
                Color::White
            }
        }
        Color::Black => {
            if black_count == 0 || black_count > 2 {
                Color::White
            } else {
                Color::Black
            }
        }
    }
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 10, 2208),
        ("test2.txt", 1, 0),
        ("test3.txt", 1, 0),
        ("input.txt", 330, 3711),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        let direction_map: HashMap<&str, Point> = vec![
            ("e", Point::new(2, 0)),
            ("se", Point::new(1, -1)),
            ("sw", Point::new(-1, -1)),
            ("w", Point::new(-2, 0)),
            ("nw", Point::new(-1, 1)),
            ("ne", Point::new(1, 1)),
        ]
        .into_iter()
        .collect();
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let re = Regex::new(r"(e|se|sw|w|nw|ne)+?").unwrap();
        let mut tiles: HashMap<Point, Color> =
            vec![(Point::new(0, 0), Color::White)].into_iter().collect();
        for line in file_content.iter() {
            let instructions: Vec<Point> = re
                .captures_iter(line)
                .map(|cap| direction_map.get(&cap[1]).unwrap().clone())
                .collect();
            let address = instructions
                .into_iter()
                .fold(Point::new(0, 0), |pt, instr| pt.add(&instr));

            tiles
                .entry(address)
                .and_modify(|x| {
                    *x = match x {
                        Color::White => Color::Black,
                        Color::Black => Color::White,
                    }
                })
                .or_insert(Color::Black);
        }
        assert_eq!(
            tiles.iter().filter(|(_, v)| **v == Color::Black).count(),
            result_1
        );

        let mut previous_tiles = tiles.clone();
        tiles.clear();
        let neighbours: Vec<Point> = direction_map.values().cloned().collect();
        for _ in 0..100 {
            for (point, color) in previous_tiles.iter() {
                let black_count =
                    get_neighbouring_black_tiles(&point, &previous_tiles, &neighbours);
                let new_color = get_new_color(black_count, *color);
                tiles.insert(point.clone(), new_color);
                if *color == Color::Black {
                    for n in neighbours.iter() {
                        let tmp_point: Point = point.add(n);
                        if previous_tiles.contains_key(&tmp_point) {
                            continue;
                        }
                        let black_count =
                            get_neighbouring_black_tiles(&tmp_point, &previous_tiles, &neighbours);
                        let new_color = get_new_color(black_count, Color::White);
                        if new_color == Color::Black {
                            tiles.insert(tmp_point, new_color);
                        }
                    }
                }
            }
            previous_tiles = tiles.clone();
        }
        assert_eq!(
            tiles.iter().filter(|(_, v)| **v == Color::Black).count(),
            result_2
        );
    }
    Ok(())
}
