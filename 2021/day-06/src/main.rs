use std::io::{self};

fn transform_input(file_content: &Vec<String>) -> Vec<usize> {
    let helper: Vec<usize> = file_content[0]
        .split(",")
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut state = vec![0; 9];
    for el in helper {
        state[el] += 1;
    }
    state
}

fn do_day(state: &mut Vec<usize>) {
    let new_to_spawn = state[0];
    for i in 1..state.len() {
        state[i - 1] = state[i];
    }
    state[8] = new_to_spawn;
    state[6] += new_to_spawn;
}

fn do_n_days(state: &mut Vec<usize>, n_days: usize) {
    for _ in 0..n_days {
        do_day(state);
    }
}

fn part_1(file_content: &Vec<String>) -> usize {
    let mut state = transform_input(&file_content);
    do_n_days(&mut state, 80);
    state.iter().sum()
}

fn part_2(file_content: &Vec<String>) -> usize {
    let mut state = transform_input(&file_content);
    do_n_days(&mut state, 256);
    state.iter().sum()
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 5934, 26984457539),
        ("input.txt", 366057, 1653559299811),
    ];
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
