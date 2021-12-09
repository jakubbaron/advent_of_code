use std::collections::HashSet;
use std::io::{self};

fn parse_input(file_content: &Vec<String>) -> Vec<Vec<i32>> {
    file_content
        .iter()
        .map(|line| {
            line.chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn get_neighbours(data: &Vec<Vec<i32>>, i: usize, j: usize) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];
    if i > 0 {
        output.push(data[i - 1][j]);
    }
    if i < data.len() - 1 {
        output.push(data[i + 1][j]);
    }
    if j > 0 {
        output.push(data[i][j - 1]);
    }
    if j < data[i].len() - 1 {
        output.push(data[i][j + 1]);
    }
    output
}

fn part_1(file_content: &Vec<String>) -> i32 {
    let data = parse_input(&file_content);
    let mut outputs: Vec<i32> = vec![];
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            let data_point = data[i][j];
            let neighbours = get_neighbours(&data, i, j);
            let filtered_neighbours = neighbours
                .iter()
                .filter(|&n| n > &data_point)
                .collect::<Vec<_>>();
            if neighbours.len() == filtered_neighbours.len() {
                outputs.push(data_point.clone());
            }
        }
    }
    // println!("Part 1 outputs: {:?}", outputs);
    outputs.iter().sum::<i32>() + outputs.len() as i32
}

fn get_neighbours_coords(data: &Vec<Vec<i32>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = vec![];
    if i > 0 {
        output.push((i - 1, j));
    }
    if i < data.len() - 1 {
        output.push((i + 1, j));
    }
    if j > 0 {
        output.push((i, j - 1));
    }
    if j < data[i].len() - 1 {
        output.push((i, j + 1));
    }
    output
}

fn visit(data: &Vec<Vec<i32>>, visited: &mut HashSet<(usize, usize)>, i: usize, j: usize) -> i32 {
    visited.insert((i, j));
    if data[i][j] == 9 {
        return 0;
    }
    let coords = get_neighbours_coords(&data, i, j);
    let mut curr_size = 1;
    for coord in coords {
        if visited.contains(&coord) {
            continue;
        }
        let (new_i, new_j) = coord;
        curr_size += visit(&data, visited, new_i, new_j);
    }
    curr_size
}

fn part_2(file_content: &Vec<String>) -> i32 {
    let data = parse_input(&file_content);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut outputs: Vec<i32> = vec![];
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if !visited.contains(&(i, j)) {
                let curr_size = visit(&data, &mut visited, i, j);
                if curr_size != 0 {
                    outputs.push(curr_size);
                }
            }
        }
    }
    // println!("Part 2 outputs: {:?}", outputs);

    outputs.sort();
    outputs[outputs.len() - 3..].iter().fold(1, |acc, val| acc*val)
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 15, 1134), ("input.txt", 512, 1600104)];
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
