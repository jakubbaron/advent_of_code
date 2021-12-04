use std::io::{self};

fn parse_input(file_content: &Vec<String>) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    let helper: Vec<&str> = file_content[0].split(",").collect();
    let numbers: Vec<i32> = helper.iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut boards: Vec<Vec<Vec<i32>>> = vec![];
    let mut temp_board: Vec<Vec<i32>> = vec![];
    for line in file_content[2..].iter() {
        if line.len() == 0 {
            boards.push(temp_board);
            temp_board = vec![];
            continue;
        }
        let helper: Vec<&str> = line.split(" ").collect();
        let row: Vec<i32> = helper
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        temp_board.push(row);
    }
    if temp_board.len() != 0 {
        boards.push(temp_board);
    }

    (numbers, boards)
}

fn check_row(row: &Vec<i32>) -> bool {
    return row.iter().sum::<i32>() == 0;
}

fn check_board(board: &Vec<Vec<i32>>) -> bool {
    for i in 0..board.len() {
        if check_row(&board[i]) {
            return true;
        }
    }
    for i in 0..board[0].len() {
        let mut tmp_row = vec![0; board[0].len()];
        for j in 0..board.len() {
            tmp_row[j] = board[j][i];
        }
        if check_row(&tmp_row) {
            return true;
        }
    }
    return false;
}

fn count_unmarked(board: &Vec<Vec<i32>>) -> i32 {
    board.iter().flat_map(|x: &Vec<_>| x.iter()).sum()
}

fn display_board(board: &Vec<Vec<i32>>) {
    for row in board.iter() {
        println!("{:?}", row);
    }
}

fn part_1(file_content: &Vec<String>) -> i32 {
    let (numbers, boards) = parse_input(&file_content);
    let mut markers = boards.clone();
    for number in numbers.iter() {
        for board_id in 0..boards.len() {
            let board = &boards[board_id];
            for i in 0..board[0].len() {
                for j in 0..board[i].len() {
                    if &board[i][j] == number {
                        markers[board_id][i][j] = 0;
                        if check_board(&markers[board_id]) {
                            let count = count_unmarked(&markers[board_id]);
                            return count * number;
                        }
                    }
                }
            }
        }
    }
    -1
}

fn part_2(file_content: &Vec<String>) -> i32 {
    let (numbers, boards) = parse_input(&file_content);
    let mut markers = boards.clone();
    let mut winner_boards: Vec<usize> = vec![];
    for number in numbers.iter() {
        for board_id in 0..boards.len() {
            if winner_boards.contains(&board_id) {
                continue;
            }
            let board = &boards[board_id];
            for i in 0..board[0].len() {
                for j in 0..board[i].len() {
                    if &board[i][j] == number {
                        markers[board_id][i][j] = 0;
                        if check_board(&markers[board_id]) {
                            winner_boards.push(board_id);
                            if winner_boards.len() == boards.len() {
                                let count = count_unmarked(&markers[board_id]);
                                return count * number;
                            }
                        }
                    }
                }
            }
        }
    }
    -1
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 4512, 1924), ("input.txt", 58374, 11377)];
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
