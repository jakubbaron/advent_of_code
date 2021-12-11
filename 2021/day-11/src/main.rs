use std::collections::HashSet;
use std::io::{self};

fn parse_input(file_content: &Vec<String>) -> Vec<Vec<i32>> {
    file_content
        .iter()
        .map(|x| {
            x.chars()
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn get_neighbours(state: &Vec<Vec<i32>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = vec![];
    let pairs: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let i = i as i32;
    let j = j as i32;
    for (ii, jj) in pairs.iter() {
        if i + ii < 0 {
            continue;
        }
        if j + jj < 0 {
            continue;
        }
        if (i + ii) as usize >= state.len() {
            continue;
        }
        if (j + jj) as usize >= state[0].len() {
            continue;
        }
        let pair = ((i + ii) as usize, (j + jj) as usize);
        output.push(pair);
    }
    output
}

fn bump_energies(state: &mut Vec<Vec<i32>>) {
    for i in 0..state.len() {
        for j in 0..state[i].len() {
            state[i][j] += 1;
        }
    }
}
fn print_state(i: usize, state: &Vec<Vec<i32>>) {
    println!("Step {}", i);
    for line in state.iter() {
        println!("{:?}", line);
    }
}

fn flash(state: &mut Vec<Vec<i32>>, flashed: &mut HashSet<(usize, usize)>, i: usize, j: usize) {
    if flashed.contains(&(i, j)) {
        return;
    }
    if state[i][j] > 9 {
        flashed.insert((i, j));
        for neighbour in get_neighbours(state, i, j) {
            let (ii, jj) = neighbour;
            state[ii][jj] += 1;
            flash(state, flashed, ii, jj);
        }
    }
}

fn iterate_flash(state: &mut Vec<Vec<i32>>) -> HashSet<(usize, usize)> {
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..state.len() {
        for j in 0..state[i].len() {
            flash(state, &mut flashed, i, j);
        }
    }
    flashed
}
fn clean_flashed(state: &mut Vec<Vec<i32>>, flashed: &HashSet<(usize, usize)>) {
    for coords in flashed.iter() {
        let (i, j) = &coords;
        state[*i][*j] = 0;
    }
}

fn part_1(file_content: &Vec<String>) -> usize {
    let mut state = parse_input(&file_content);
    let mut count = 0;
    for _ in 0..100 {
        bump_energies(&mut state);
        let flashed = iterate_flash(&mut state);
        count += flashed.len();
        clean_flashed(&mut state, &flashed)
    }
    count
}

fn part_2(file_content: &Vec<String>) -> i32 {
    let mut state = parse_input(&file_content);
    let mut step = 0;
    let total_count = state.len() * state[0].len();
    loop {
        step += 1;
        bump_energies(&mut state);
        let flashed = iterate_flash(&mut state);
        clean_flashed(&mut state, &flashed);
        if flashed.len() == total_count {
            break;
        }
    }
    step
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 1656, 195), ("input.txt", 1688, 403)];
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
