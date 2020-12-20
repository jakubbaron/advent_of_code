use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Side {
    Bottom,
    Top,
    Left,
    Right,
}

fn get_opposite_side(side: &Side) -> Side {
    match side {
        Side::Bottom => Side::Top,
        Side::Right => Side::Left,
        Side::Top => Side::Bottom,
        Side::Left => Side::Left,
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Frame {
    title: String,
    pub frame_no: usize,
    data: Vec<Vec<char>>,
    sides: HashMap<Side, Vec<char>>,
    neighbours: HashMap<Side, Neighbour>,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Neighbour {
    frame_no: usize,
}

fn get_bottom(data: &Vec<Vec<char>>) -> Vec<char> {
    data[data.len() - 1].to_vec()
}
fn get_top(data: &Vec<Vec<char>>) -> Vec<char> {
    data[0].to_vec()
}
fn get_left(data: &Vec<Vec<char>>) -> Vec<char> {
    data.iter().map(|row| row[0]).collect()
}
fn get_right(data: &Vec<Vec<char>>) -> Vec<char> {
    data.iter().map(|row| row[data.len() - 1]).collect()
}
fn rotate(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_data: Vec<Vec<char>> = vec![vec!['.'; data.len()]; data[0].len()];
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            new_data[i][j] = data[data.len() - j - 1][i];
        }
    }
    new_data
}

fn flip(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_data: Vec<Vec<char>> = vec![vec!['.'; data.len()]; data[0].len()];
    for i in 0..data.len() {
        for j in 0..data.len() {
            new_data[i][j] = data[i][data.len() - j - 1];
        }
    }
    new_data
}

fn get_sides(data: &Vec<Vec<char>>) -> HashMap<Side, Vec<char>> {
    let bottom = get_bottom(&data);
    let top = get_top(&data);
    let left = get_left(&data);
    let right = get_right(&data);
    vec![
        (Side::Bottom, bottom),
        (Side::Top, top),
        (Side::Left, left),
        (Side::Right, right),
    ]
    .into_iter()
    .collect()
}

impl Frame {
    pub fn new(vec: &Vec<Vec<char>>) -> Frame {
        assert!(vec.len() > 2);
        assert!(vec[0].len() > 2);
        let title: String = vec[0].iter().collect();
        let tmp: Vec<&str> = title.split(" ").collect();
        let frame_no = tmp[1].replace(":", " ").trim().parse::<usize>().unwrap();
        let data = vec[1..vec.len()].to_vec();
        assert_eq!(data.len(), 10);
        assert_eq!(data[0].len(), 10);
        let sides = get_sides(&data);
        Frame {
            title,
            frame_no,
            data,
            sides,
            neighbours: HashMap::new(),
        }
    }
    fn get_side(&self, side: &Side) -> &Vec<char> {
        self.sides.get(side).unwrap()
    }
    fn get_neighbour_frame_no(&self, side: &Side) -> usize {
        self.neighbours.get(side).unwrap().frame_no
    }
    pub fn get_neighbours_len(&self) -> usize {
        self.neighbours.len()
    }
    fn has_neighbour(&self, side: &Side) -> bool {
        self.neighbours.contains_key(side)
    }
    fn get_neighbours_directions(&self) -> Vec<Side> {
        self.neighbours.keys().cloned().collect()
    }

    fn is_top_left(&self) -> bool {
        self.neighbours.contains_key(&Side::Bottom) && self.neighbours.contains_key(&Side::Right)
    }

    fn data_without_borders(&self) -> Vec<Vec<char>> {
        self.data[1..self.data.len() - 1]
            .iter()
            .map(|row| row[1..row.len() - 1].to_vec())
            .collect()
    }

    fn flip(&self) -> Frame {
        let new_data = flip(&self.data);
        let sides = get_sides(&new_data);
        let mut new_neighbours: HashMap<Side, Neighbour> = HashMap::new();
        for (k, n) in self.neighbours.iter() {
            let new_side = match k {
                Side::Right => Side::Left,
                Side::Left => Side::Right,
                _ => k.clone(),
            };
            new_neighbours.insert(new_side, n.clone());
        }
        Frame {
            title: self.title.to_string(),
            frame_no: self.frame_no,
            data: new_data,
            sides,
            neighbours: new_neighbours,
        }
    }

    fn rotate(&self) -> Frame {
        let new_data = rotate(&self.data);
        let sides = get_sides(&new_data);
        let mut new_neighbours: HashMap<Side, Neighbour> = HashMap::new();
        for (k, n) in self.neighbours.iter() {
            let new_side = match k {
                Side::Top => Side::Right,
                Side::Right => Side::Bottom,
                Side::Bottom => Side::Left,
                Side::Left => Side::Top,
            };
            new_neighbours.insert(new_side, n.clone());
        }
        Frame {
            title: self.title.to_string(),
            frame_no: self.frame_no,
            data: new_data,
            sides,
            neighbours: new_neighbours,
        }
    }


    fn try_neighbour(&mut self, neighbour: &mut Frame) -> bool {
        if self.neighbours.len() == 4 || neighbour.neighbours.len() == 4 {
            return false;
        }
        let sides = vec![Side::Left, Side::Right, Side::Bottom, Side::Top];
        for side in sides.iter() {
            if self.neighbours.contains_key(side) {
                continue;
            }
            let frame_side = self.sides.get(side).unwrap();
            for match_side in sides.iter() {
                if neighbour.neighbours.contains_key(match_side) {
                    continue;
                }
                let their_side = neighbour.sides.get(match_side).unwrap();
                let reverse_side: Vec<char> = their_side.to_vec().into_iter().rev().collect();
                if frame_side == their_side || frame_side == &reverse_side {
                    self.add_neighbour(side, neighbour.frame_no);
                    neighbour.add_neighbour(match_side, self.frame_no);
                    return true;
                }
            }
        }
        return false;
    }

    fn add_neighbour(&mut self, side: &Side, frame_no: usize) {
        self.neighbours.insert(side.clone(), Neighbour { frame_no });
    }
}

fn find_monsters(end_data: &Vec<Vec<char>>, monster: &Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
    let mut picture_has_monster = false;
    let mut data_with_monster: Vec<Vec<char>> = end_data.to_vec();
    let monster_row_len = monster[0].len();
    let mut row_id = 0;
    while row_id < end_data.len() - monster.len() {
        let mut col_id = 0;
        while col_id < end_data[0].len() - monster_row_len {
            let mut found_monster = true;
            'monster_window: for (monster_row, row) in monster
                .iter()
                .zip(end_data[row_id..row_id + monster.len()].iter())
            {
                for (monster_ch, ch) in monster_row
                    .iter()
                    .zip(row[col_id..col_id + monster_row_len].iter())
                {
                    if *monster_ch == '#' && *ch != '#' {
                        found_monster = false;
                        break 'monster_window;
                    }
                }
            }
            if found_monster {
                for (row_offset, (monster_row, row)) in monster
                    .iter()
                    .zip(end_data[row_id..row_id + monster.len()].iter())
                    .enumerate()
                {
                    for (col_offset, (monster_ch, ch)) in monster_row
                        .iter()
                        .zip(row[col_id..col_id + monster_row_len].iter())
                        .enumerate()
                    {
                        if *monster_ch == '#' && *ch == '#' {
                            data_with_monster[row_id + row_offset][col_id + col_offset] = 'O';
                        }
                    }
                }
                col_id += monster_row_len;
                picture_has_monster = true;
            } else {
                col_id += 1;
            }
        }
        row_id += 1;
    }
    (picture_has_monster, data_with_monster)
}

fn draw_picture(data: &Vec<Vec<char>>) {
    for row in data.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn measure_roughness(data: &Vec<Vec<char>>) -> usize {
    data.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&x| *x == '#').count()
    })
}

pub fn check_monsters(end_data: &Vec<Vec<char>>, monster: &Vec<Vec<char>>) -> usize {
    let mut end_data = end_data.to_vec();
    let mut res_2 = 0;
    for _ in 0..4 {
        let (picture_has_monster, data_with_monster) = find_monsters(&end_data, &monster);
        if picture_has_monster {
            draw_picture(&data_with_monster);
            res_2 = measure_roughness(&data_with_monster);
            break;
        }
        let flipped_data = flip(&end_data);
        let (picture_has_monster, data_with_monster) = find_monsters(&flipped_data, &monster);
        if picture_has_monster {
            draw_picture(&data_with_monster);
            res_2 = measure_roughness(&data_with_monster);
            break;
        }

        end_data = rotate(&end_data);
    }
    res_2
}

pub fn match_neighbours(frames: &HashMap<usize, RefCell<Frame>>) {
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
            frame_1.try_neighbour(&mut frame_2);
        }
    }
}

fn find_top_left_corner(frames: &HashMap<usize, RefCell<Frame>>) -> Frame {
   let mut first_corner: Option<Frame> = None;
   for frame in frames.values() {
       let frame = frame.borrow();
       if frame.get_neighbours_len() == 2 {
           let mut frame = frame.clone();
           while !frame.is_top_left() {
               frame = frame.rotate();
           }
           first_corner = Some(frame.clone());
       }
   }
   first_corner.unwrap()
}

pub fn create_picture(frames: &HashMap<usize, RefCell<Frame>>) -> Vec<Vec<Option<Frame>>> {
    let first_corner = find_top_left_corner(frames);
    let directions = first_corner.get_neighbours_directions();
    let mut queue: Vec<(Frame, usize, usize)> = vec![(first_corner, 0, 0)];
    let mut seen: HashSet<usize> = HashSet::new();
    let sq_size = (frames.len() as f64).sqrt() as usize;
    let mut end_vec: Vec<Vec<Option<Frame>>> = vec![vec![None; sq_size]; sq_size];
    while !queue.is_empty() {
        let (current, i, j) = queue.pop().unwrap();
        if seen.contains(&current.frame_no) {
            continue;
        }
        seen.insert(current.frame_no);
        for dir in directions.iter() {
            if !current.has_neighbour(&dir) {
                continue;
            }
            let neigh_no = current.get_neighbour_frame_no(&dir);
            if seen.contains(&neigh_no) {
                continue;
            }
            let my_side = current.get_side(&dir);
            let other_direction = get_opposite_side(&dir);

            let mut neigh: Frame = frames.get(&neigh_no).unwrap().borrow().clone();
            let mut found = false;
            for _ in 0..4 {
                let other_side = neigh.get_side(&other_direction);
                if other_side == my_side {
                    found = true;
                    break;
                }
                let flipped = neigh.flip();
                let flipped_side = flipped.get_side(&other_direction);
                if flipped_side == my_side {
                    neigh = flipped;
                    found = true;
                    break;
                }
                neigh = neigh.rotate();
            }
            if !found {
                continue;
            }
            if dir == &Side::Bottom {
                queue.push((neigh, i + 1, j));
            } else {
                queue.push((neigh, i, j + 1));
            }
        }
        end_vec[i][j] = Some(current);
    }
    end_vec
}
pub fn print_picture(end_vec: &Vec<Vec<Option<Frame>>>) {
    for row in end_vec.iter() {
        for f in row.iter() {
            match f {
                Some(f) => print!("{}, ", f.frame_no),
                None => print!("0"),
            };
        }
        println!("");
    }
}

pub fn frames_to_picture(end_vec: Vec<Vec<Option<Frame>>>) -> Vec<Vec<char>> {
    let row_len = end_vec.first().unwrap().len();
    let col_len = end_vec.len();
    let data_size = match &end_vec[0][0] {
        Some(f) => f.data_without_borders().len(),
        None => panic!("Empty data!"),
    };

    let mut end_data: Vec<Vec<char>> =
        vec![vec!['.'; row_len * data_size]; col_len * data_size];
    for (i, row) in end_vec.into_iter().enumerate() {
        for (j, f) in row.into_iter().enumerate() {
            match f {
                Some(f) => {
                    for (ii, data_row) in f.data_without_borders().into_iter().enumerate() {
                        for (jj, ch) in data_row.into_iter().enumerate() {
                            end_data[i * data_size + ii][j * data_size + jj] = ch;
                        }
                    }
                }
                None => panic!("Empty place in the picture!"),
            };
        }
    }
    end_data
}
