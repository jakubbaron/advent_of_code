use std::collections::HashSet;
use std::io::{self};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn get_neighbours(&self) -> HashSet<Point> {
        let mut neighbours: HashSet<Point> = HashSet::with_capacity(26);
        for tx in self.x - 1..=self.x + 1 {
            for ty in self.y - 1..=self.y + 1 {
                for tz in self.z - 1..=self.z + 1 {
                    neighbours.insert(Point {
                        x: tx,
                        y: ty,
                        z: tz,
                    });
                }
            }
        }
        neighbours.remove(self);
        neighbours
    }
}


#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct NewPoint {
    coords: Vec<i32>,
}

impl NewPoint {
    fn get_neighbours(&self) -> HashSet<NewPoint> {
        let cap = 3_usize.pow(self.coords.len() as u32);
        let mut neighbours: HashSet<NewPoint> = HashSet::with_capacity(cap);
        neighbours.insert(self.clone());
        for dimension in 0..self.coords.len() {
            let mut new_set: HashSet<NewPoint> = HashSet::with_capacity(cap);
            for p in neighbours.iter() {
                for modifier in vec![-1, 0, 1] {
                    let mut tmp = p.clone();
                    tmp.coords[dimension] = p.coords[dimension] + modifier;
                    new_set.insert(tmp);
                }
            }
            neighbours = new_set;
        }

        neighbours.remove(self);
        neighbours
    }
}

fn generate_empty_points(dimension: i32, current_size: i32) -> HashSet<NewPoint> {
    let cap = 3_usize.pow(dimension as u32);
    let mut neighbours: HashSet<NewPoint> = HashSet::with_capacity(cap);
    for i in -dimension..=dimension {
        let mut t = vec![0; dimension as usize];
        t[0] = i;
        neighbours.insert(NewPoint{coords:t});
    }
    for d in 0..dimension {
        let mut new_set: HashSet<NewPoint> = HashSet::with_capacity(cap);
        for p in neighbours.iter() {
            for modifier in -current_size..=current_size {
                let mut tmp = p.clone();
                let d = d as usize;
                tmp.coords[d] = p.coords[d] + modifier;
                new_set.insert(tmp);
            }
        }
        neighbours = new_set;
    }
    neighbours
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point4d {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point4d {
    fn get_neighbours(&self) -> Vec<Point4d> {
        let mut neighbours: Vec<Point4d> = Vec::new();
        for tx in self.x - 1..=self.x + 1 {
            for ty in self.y - 1..=self.y + 1 {
                for tz in self.z - 1..=self.z + 1 {
                    for tw in self.w - 1..=self.w + 1 {
                        if tx == self.x && ty == self.y && tz == self.z && tw == self.w {
                            continue;
                        }
                        neighbours.push(Point4d {
                            x: tx,
                            y: ty,
                            z: tz,
                            w: tw,
                        });
                    }
                }
            }
        }
        neighbours
    }
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 112, 848), ("input.txt", 273, 1504)];
    for (f, result_1, result_2) in files_results.iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();

        let mut previous_cube: HashSet<NewPoint> = HashSet::new();
        for (y, line) in file_content.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    let y = y as i32;
                    let x = x as i32;
                    let z = 0_i32;
                    previous_cube.insert(NewPoint { coords: vec![x, y, z]});
                }
            }
        }
        println! {"{:?}", previous_cube};
        let mut current_size = file_content.len() as i32;
        let dimension = 3_i32;
        for _ in 1..=6 {
            let mut new_cube: HashSet<NewPoint> = HashSet::new();
            for p in generate_empty_points(dimension, current_size).iter() {
                let active = previous_cube.intersection(&p.get_neighbours()).count();
                if previous_cube.contains(&p) {
                    if active == 2 || active == 3 {
                        new_cube.insert(p.clone());
                    }
                } else {
                    if active == 3 {
                        new_cube.insert(p.clone());
                    }
                }
            }
            previous_cube = new_cube;
            current_size += 1;
        }
        println!("Actives 3d {}", previous_cube.len());
        assert_eq!(previous_cube.len(), *result_1);

        let mut previous_cube: HashSet<NewPoint> = HashSet::new();
        for (y, line) in file_content.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    let y = y as i32;
                    let x = x as i32;
                    let z = 0_i32;
                    let w = 0_i32;
                    previous_cube.insert(NewPoint{ coords:vec![x, y, z, w] });
                }
            }
        }
        println! {"{:?}", previous_cube};
        let mut current_size = file_content.len() as i32;
        let dimension = 4_i32;
        for _ in 1..=6 {
            let mut new_cube: HashSet<NewPoint> = HashSet::new();
            for p in generate_empty_points(dimension, current_size).iter() {
                let active = previous_cube.intersection(&p.get_neighbours()).count();
                if previous_cube.contains(&p) {
                    if active == 2 || active == 3 {
                        new_cube.insert(p.clone());
                    }
                } else {
                    if active == 3 {
                        new_cube.insert(p.clone());
                    }
                }
            }
            previous_cube = new_cube;
            current_size += 1;
        }
        println!("Actives 4d {}", previous_cube.len());
        assert_eq!(previous_cube.len(), *result_2);
    }
    Ok(())
}
