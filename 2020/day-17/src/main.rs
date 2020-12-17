use std::collections::HashSet;
use std::io::{self};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3d {
    fn get_neighbours(&self) -> Vec<Point3d> {
        let mut neighbours: Vec<Point3d> = Vec::new();
        for tx in self.x - 1..=self.x + 1 {
            for ty in self.y - 1..=self.y + 1 {
                for tz in self.z - 1..=self.z + 1 {
                    if tx == self.x && ty == self.y && tz == self.z {
                        continue;
                    }
                    neighbours.push(Point3d {
                        x: tx,
                        y: ty,
                        z: tz,
                    });
                }
            }
        }
        neighbours
    }
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
        let mut previous_cube: HashSet<Point3d> = HashSet::new();
        for (y, line) in file_content.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    let y = y as i32;
                    let x = x as i32;
                    let z = 0_i32;
                    previous_cube.insert(Point3d { x, y, z });
                }
            }
        }
        println! {"{:?}", previous_cube};
        let mut current_size = file_content.len() as i32;
        for _ in 1..=6 {
            let mut new_cube: HashSet<Point3d> = HashSet::new();
            for x in -current_size..=current_size {
                for y in -current_size..=current_size {
                    for z in -current_size..=current_size {
                        let x = x as i32;
                        let y = y as i32;
                        let z = z as i32;
                        let p = Point3d { x, y, z };
                        let mut active = 0;
                        for n in p.get_neighbours().iter() {
                            if previous_cube.contains(&n) {
                                active += 1;
                            }
                        }
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
                }
            }
            previous_cube = new_cube;
            current_size += 1;
        }
        println!("Actives 3d {}", previous_cube.len());
        assert_eq!(previous_cube.len(), *result_1);

        let mut previous_cube: HashSet<Point4d> = HashSet::new();
        for (y, line) in file_content.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    let y = y as i32;
                    let x = x as i32;
                    let z = 0_i32;
                    let w = 0_i32;
                    previous_cube.insert(Point4d { x, y, z, w });
                }
            }
        }
        println! {"{:?}", previous_cube};
        let mut current_size = file_content.len() as i32;
        for _ in 1..=6 {
            let mut new_cube: HashSet<Point4d> = HashSet::new();
            for x in -current_size..=current_size {
                for y in -current_size..=current_size {
                    for z in -current_size..=current_size {
                        for w in -current_size..=current_size {
                            let x = x as i32;
                            let y = y as i32;
                            let z = z as i32;
                            let w = w as i32;
                            let p = Point4d { x, y, z, w };
                            let mut active = 0;
                            for n in p.get_neighbours().iter() {
                                if previous_cube.contains(&n) {
                                    active += 1;
                                }
                            }
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
