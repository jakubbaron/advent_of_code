use std::cmp;
use std::io::{self};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn add_point(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    fn do_step(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        } else if self.x < 0 {
            self.x += 1;
        }
        self.y -= 1;
    }
}

#[derive(Debug, Clone, Copy)]
struct Area {
    bottom_left: Point,
    top_right: Point,
}

impl Area {
    fn point_in_area(&self, p: &Point) -> bool {
        p.x >= self.bottom_left.x
            && p.x <= self.top_right.x
            && p.y >= self.bottom_left.y
            && p.y <= self.top_right.y
    }
    fn is_overshoot(&self, p: &Point, velocity: &Point) -> bool {
        if velocity.x > 0 {
            p.x > self.top_right.x || p.y < self.bottom_left.y
        } else if velocity.x < 0 {
            p.x < self.top_right.x || p.y < self.bottom_left.y
        } else {
            p.y < self.bottom_left.y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_in_area() {
        let bottom_left = Point { x: 20, y: -10 };
        let top_right = Point { x: 30, y: -5 };
        let area = Area {
            bottom_left,
            top_right,
        };
        let p = Point { x: 20, y: -5 };
        assert!(area.point_in_area(&p));
        assert!(area.point_in_area(&bottom_left));
        assert!(area.point_in_area(&top_right));
    }

    #[test]
    fn point_not_in_area() {
        let bottom_left = Point { x: 20, y: -10 };
        let top_right = Point { x: 30, y: -5 };
        let area = Area {
            bottom_left,
            top_right,
        };
        let p = Point { x: 19, y: -5 };
        assert!(!area.point_in_area(&p));
    }
}

fn get_numbers(s: &str) -> (i32, i32) {
    let splitted = s.split("..").collect::<Vec<_>>();
    let idx = splitted[0].chars().position(|x| x == '=').unwrap();
    let first = splitted[0][idx + 1..].parse::<i32>().unwrap();
    let second = splitted[1].parse::<i32>().unwrap();
    (first, second)
}

fn get_area(file_content: &Vec<String>) -> Area {
    let line = &file_content[0];
    let splitted = line.split(", ").collect::<Vec<_>>();
    let (x1, x2) = get_numbers(&splitted[0]);
    let (y1, y2) = get_numbers(&splitted[1]);
    let bottom_left = Point { x: x1, y: y1 };
    let top_right = Point { x: x2, y: y2 };
    Area {
        bottom_left,
        top_right,
    }
}

fn part_1(file_content: &Vec<String>) -> i32 {
    let area = get_area(&file_content);
    let mut max_height = i32::MIN;
    for x in -1000..1000 {
        for y in -1000..1000 {
            let mut p = Point { x: 0, y: 0 };
            let mut velocity = Point { x, y };
            let mut curr_max_height = i32::MIN;
            while !area.is_overshoot(&p, &velocity) {
                p.add_point(&velocity);
                curr_max_height = std::cmp::max(curr_max_height, p.y);
                velocity.do_step();
                if area.point_in_area(&p) {
                    break;
                }
            }
            if area.point_in_area(&p) {
                max_height = std::cmp::max(max_height, curr_max_height);
            }
        }
    }
    max_height
}

fn part_2(file_content: &Vec<String>) -> i32 {
    let area = get_area(&file_content);
    let mut times_in_area = 0;
    for x in -1000..1000 {
        for y in -1000..1000 {
            let mut p = Point { x: 0, y: 0 };
            let mut velocity = Point { x, y };
            while !area.is_overshoot(&p, &velocity) {
                p.add_point(&velocity);
                velocity.do_step();
                if area.point_in_area(&p) {
                    times_in_area += 1;
                    break;
                }
            }
        }
    }
    times_in_area
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 45, 112), ("input.txt", 9730, 0)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("{}", f);
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
