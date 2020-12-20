use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::{self};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Side {
    Bottom,
    Top,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Frame {
    title: String,
    frame_no: usize,
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

impl Frame {
    fn new(vec: &Vec<Vec<char>>) -> Frame {
        assert!(vec.len() > 2);
        assert!(vec[0].len() > 2);
        let title: String = vec[0].iter().collect();
        let tmp: Vec<&str> = title.split(" ").collect();
        let frame_no = tmp[1].replace(":", " ").trim().parse::<usize>().unwrap();
        let data = vec[1..vec.len()].to_vec();
        assert_eq!(data.len(), 10);
        assert_eq!(data[0].len(), 10);
        let bottom = get_bottom(&data);
        let top = get_top(&data);
        let left = get_left(&data);
        let right = get_right(&data);
        let sides: HashMap<Side, Vec<char>> = vec![
            (Side::Bottom, bottom),
            (Side::Top, top),
            (Side::Left, left),
            (Side::Right, right),
        ]
        .into_iter()
        .collect();
        Frame {
            title,
            frame_no,
            data,
            sides,
            neighbours: HashMap::new(),
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
            for match_side in sides.iter() {
                if neighbour.neighbours.contains_key(match_side) {
                    continue;
                }
                let their_side = neighbour.sides.get(match_side).unwrap();
                let frame_side = self.sides.get(side).unwrap();
                if frame_side == their_side {
                    self.add_neighbour(
                        side,
                        Neighbour {
                            frame_no: neighbour.frame_no,
                        },
                    );
                    neighbour.add_neighbour(
                        match_side,
                        Neighbour {
                            frame_no: self.frame_no,
                        },
                    );
                    return true;
                }
                let reverse: Vec<char> = their_side.to_vec().into_iter().rev().collect();
                if frame_side == &reverse {
                    self.add_neighbour(
                        side,
                        Neighbour {
                            frame_no: neighbour.frame_no,
                        },
                    );
                    neighbour.add_neighbour(
                        match_side,
                        Neighbour {
                            frame_no: self.frame_no,
                        },
                    );
                    return true;
                }
            }
        }
        return false;
    }

    fn data_without_borders(&self) -> Vec<Vec<char>> {
        self.data[1..self.data.len() - 1]
            .iter()
            .map(|row| row[1..row.len() - 1].to_vec())
            .collect()
    }
    fn add_neighbour(&mut self, side: &Side, neighbour: Neighbour) {
        self.neighbours.insert(side.clone(), neighbour);
    }

    fn flip(&self) -> Frame {
        let new_data = flip(&self.data);
        let bottom = get_bottom(&new_data);
        let top = get_top(&new_data);
        let left = get_left(&new_data);
        let right = get_right(&new_data);
        let sides: HashMap<Side, Vec<char>> = vec![
            (Side::Bottom, bottom),
            (Side::Top, top),
            (Side::Left, left),
            (Side::Right, right),
        ]
        .into_iter()
        .collect();
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
        let bottom = get_bottom(&new_data);
        let top = get_top(&new_data);
        let left = get_left(&new_data);
        let right = get_right(&new_data);
        let sides: HashMap<Side, Vec<char>> = vec![
            (Side::Bottom, bottom),
            (Side::Top, top),
            (Side::Left, left),
            (Side::Right, right),
        ]
        .into_iter()
        .collect();
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
            for (monster_row, row) in monster
                .iter()
                .zip(end_data[row_id..row_id + monster.len()].iter())
            {
                for (monster_ch, ch) in monster_row
                    .iter()
                    .zip(row[col_id..col_id + monster_row_len].iter())
                {
                    if *monster_ch == '#' && *ch != '#' {
                        found_monster = false;
                        break;
                    }
                }
                if !found_monster {
                    break;
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

fn check_monsters(end_data: &Vec<Vec<char>>, monster: &Vec<Vec<char>>) -> usize {
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

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 20899048083289_usize, 273),
        ("input.txt", 20913499394191, 2209),
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
            } else {
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
                frame_1.try_neighbour(&mut frame_2);
            }
        }
        let res_1 = frames
            .values()
            .map(|x| x.borrow())
            .filter(|x| x.neighbours.len() == 2)
            .fold(1, |acc, frame| acc * frame.frame_no);
        assert_eq!(res_1, *result_1);

        let mut first_corner: Option<Frame> = None;
        for frame in frames.values() {
            let frame = frame.borrow();
            if frame.neighbours.len() == 2 {
                let mut frame = frame.clone();
                while !frame.neighbours.contains_key(&Side::Bottom)
                    || !frame.neighbours.contains_key(&Side::Right)
                {
                    frame = frame.rotate();
                }
                first_corner = Some(frame.clone());
            }
        }
        let first_corner = first_corner.unwrap().clone();

        let directions: Vec<Side> = first_corner.neighbours.keys().cloned().collect();
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
            // println!("Current: {:?}", current.frame_no);
            for dir in directions.iter() {
                if !current.neighbours.contains_key(&dir) {
                    // println!("Frame: {} doesn't have {:?}", current.frame_no, dir);
                    continue;
                }
                let neigh_no = current.neighbours.get(&dir).unwrap().frame_no;
                if seen.contains(&neigh_no) {
                    continue;
                }
                let my_side = current.sides.get(&dir).unwrap();
                let other_direction = match dir {
                    Side::Right => Side::Left,
                    Side::Bottom => Side::Top,
                    _ => continue,
                };
                let mut neigh: Frame = frames.get(&neigh_no).unwrap().borrow().clone();
                let mut found = false;
                for _ in 0..4 {
                    let other_side = neigh.sides.get(&other_direction).unwrap();
                    if other_side == my_side {
                        found = true;
                        break;
                    }
                    let flipped = neigh.flip();
                    let flipped_side = flipped.sides.get(&other_direction).unwrap();
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
        for row in end_vec.iter() {
            for f in row.iter() {
                match f {
                    Some(f) => print!("{}, ", f.frame_no),
                    None => panic!("Not a full frame!"),
                };
            }
            println!("");
        }

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
        let monster: Vec<Vec<char>> = std::fs::read_to_string("monster.txt")?
            .lines()
            .map(|x| x.chars().collect())
            .collect();
        let res_2 = check_monsters(&end_data, &monster);
        assert_eq!(res_2, *result_2);
    }
    Ok(())
}
