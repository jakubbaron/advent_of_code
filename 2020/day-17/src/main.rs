use std::collections::HashSet;
use std::io::{self};

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
        for p in neighbours.into_iter() {
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


fn file_content_to_new_points(file_content: &Vec<String>, dimension: usize) -> HashSet<NewPoint> {
    let cap = 3_usize.pow(dimension as u32);
    let mut previous_cube: HashSet<NewPoint> = HashSet::with_capacity(cap);
    for (y, line) in file_content.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                let y = y as i32;
                let x = x as i32;
                let mut t = vec![0; dimension];
                t[0] = x;
                t[1] = y;
                previous_cube.insert(NewPoint { coords: t});
            }
        }
    }
    previous_cube
}

fn run_simulation(mut previous_cube: HashSet<NewPoint>, dimension: i32, mut current_size: i32) -> usize {
    for _ in 1..=6 {
        let mut new_cube: HashSet<NewPoint> = HashSet::new();
        for p in generate_empty_points(dimension, current_size).into_iter() {
            let active = previous_cube.intersection(&p.get_neighbours()).count();
            if previous_cube.contains(&p) {
                if active == 2 || active == 3 {
                    new_cube.insert(p);
                }
            } else {
                if active == 3 {
                    new_cube.insert(p);
                }
            }
        }
        previous_cube = new_cube;
        current_size += 1;
    }
    previous_cube.len()
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 112, 848), ("input.txt", 273, 1504)];
    for (f, result_1, result_2) in files_results.iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();

        let dimension = 3_i32;
        let previous_cube = file_content_to_new_points(&file_content, dimension as usize);
        let current_size = file_content.len() as i32;
        let cube_1= run_simulation(previous_cube, dimension, current_size);
        println!("Actives 3d {}", cube_1);
        assert_eq!(cube_1, *result_1);

        let dimension = 4_i32;
        let previous_cube = file_content_to_new_points(&file_content, dimension as usize);
        let cube_2= run_simulation(previous_cube, dimension, current_size);
        println!("Actives 4d {}", cube_2);
        assert_eq!(cube_2, *result_2);
    }
    Ok(())
}
