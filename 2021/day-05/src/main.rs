use std::collections::HashMap;
use std::io::{self};

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn point_from_vec(vec: &Vec<i32>) -> Point {
    Point {
        x: vec[0],
        y: vec[1],
    }
}

#[derive(Debug)]
struct Line {
    p1: Optional<Point>,
    p2: Point,
}

impl Line {
    fn is_straight_line(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }
    fn mark_map(&self, hashmap: &mut HashMap<Point, i32>) {
        let (beginning_x, ending_x) = if self.p1.x < self.p2.x {
            (self.p1.x, self.p2.x)
        } else {
            (self.p2.x, self.p1.x)
        };

        let (beginning_y, ending_y) = if self.p1.y < self.p2.y {
            (self.p1.y, self.p2.y)
        } else {
            (self.p2.y, self.p1.y)
        };
        for x in beginning_x..ending_x + 1 {
            for y in beginning_y..ending_y + 1 {
                let p = Point { x, y };
                let entry = hashmap.entry(p).or_insert(0);
                *entry += 1;
            }
        }
    }
    fn mark_diagonal(&self, hashmap: &mut HashMap<Point, i32>) {
        let step_x = if self.p1.x < self.p2.x { 1 } else { -1 };
        let step_y = if self.p1.y < self.p2.y { 1 } else { -1 };
        let Point { mut x, mut y } = self.p1;
        while x != self.p2.x {
            while y != self.p2.y {
                let p = Point { x, y };
                let entry = hashmap.entry(p).or_insert(0);
                *entry += 1;
                x += step_x;
                y += step_y;
            }
        }
        let p = Point { x, y };
        let entry = hashmap.entry(p).or_insert(0);
        *entry += 1;
    }
}

fn line_from_str_line(str_line: &String) -> Line {
    let splitted_coords: Vec<_> = str_line.split(" -> ").collect();
    let str_coords_1: Vec<_> = splitted_coords[0].split(",").collect();
    let str_coords_2: Vec<_> = splitted_coords[1].split(",").collect();
    let coords1 = str_coords_1
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let coords2 = str_coords_2
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    Line {
        p1: point_from_vec(&coords1),
        p2: point_from_vec(&coords2),
    }
}

fn part_1(file_content: &Vec<String>) -> usize {
    let lines: Vec<Line> = file_content
        .iter()
        .map(|x| line_from_str_line(&x))
        .collect();
    let mut hashmap: HashMap<Point, i32> = HashMap::new();
    lines
        .iter()
        .filter(|line| line.is_straight_line())
        .for_each(|line| line.mark_map(&mut hashmap));

    hashmap.values().filter(|&x| *x > 1).count()
}

fn part_2(file_content: &Vec<String>) -> usize {
    let lines: Vec<Line> = file_content
        .iter()
        .map(|x| line_from_str_line(&x))
        .collect();

    let mut hashmap: HashMap<Point, i32> = HashMap::new();
    lines
        .iter()
        .filter(|line| line.is_straight_line())
        .for_each(|line| line.mark_map(&mut hashmap));
    lines
        .iter()
        .filter(|line| !line.is_straight_line())
        .for_each(|line| line.mark_diagonal(&mut hashmap));

    hashmap.values().filter(|&x| *x > 1).count()
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 5, 12), ("input.txt", 6461, 18065)];
    for (f, result_1, result_2) in files_results.into_iter() {
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let res_1 = part_1(&file_content);
        assert_eq!(res_1, result_1);

        let res_2 = part_2(&file_content);
        assert_eq!(res_2, result_2);
    }
    Ok(())
}
