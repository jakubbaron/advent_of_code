fn print_game(game: &Vec<i32>, curr_id: usize) {
    print!("cups: ");
    for (idd, item) in game.iter().enumerate() {
        if idd == curr_id {
            print!("({}) ", item);
        } else {
            print!("{} ", item);
        }
    }
    println!("");
}

fn main() {
    let files_results = vec![("389125467", "67384529", "1"), ("315679824", "72496583", "1")];
    for (input, result_1, result_2) in files_results.into_iter() {
        let mut game: Vec<i32> = input
            .chars()
            .map(|c| c.to_string().parse::<i32>().unwrap())
            .collect();
        let max_cup = *game.iter().max().unwrap();
        let game_len = game.len();
        let wrap_id = |i: usize| -> usize { i % game_len };
        let mut curr_id = 0;
        for id in 0..100 {
            curr_id = wrap_id(id);
            let pick_up_ids: Vec<usize> = vec![wrap_id(id + 1), wrap_id(id + 2), wrap_id(id + 3)];
            let current_cup = game[curr_id];
            let pick_up_cups: Vec<i32> = pick_up_ids.iter().map(|&i| game[i]).collect();
            let mut destination_cup = current_cup - 1;
            if destination_cup == 0 {
                destination_cup = max_cup;
            }
            while pick_up_cups.contains(&destination_cup) {
                destination_cup -= 1;
                if destination_cup == 0 {
                    destination_cup = max_cup;
                }
            }
            println!("");
            println!("-- move {} -- ", id + 1);
            print_game(&game, curr_id);
            println!("pick up: {:?}", pick_up_cups);
            println!("destination: {}", destination_cup);
            let pos = game.iter().position(|&x| x == destination_cup).unwrap();
            let mut tmp_pos = *pick_up_ids.last().unwrap();
            let mut to_move: Vec<usize> = Vec::new();
            while tmp_pos != pos {
                tmp_pos = wrap_id(tmp_pos + 1);
                to_move.push(tmp_pos);
            }
            let game_cp = game.to_vec();
            let offsets = 1..=to_move.len();
            for (i, offset) in to_move.into_iter().zip(offsets.into_iter()) {
                game[wrap_id(curr_id + offset)] = game_cp[i];
            }
            let pos = pos + game_len;
            let destination_ids = vec![wrap_id(pos-2), wrap_id(pos-1), wrap_id(pos)];
            for (pick_up_cup, destination_id) in pick_up_cups.into_iter().zip(destination_ids.into_iter()) {
                game[destination_id] = pick_up_cup;
            }
        }
        print_game(&game, wrap_id(curr_id+1));
        let pos = game.iter().position(|&x| x == 1).unwrap();
        let mut tmp_pos = pos + 1;
        let mut end_vec: Vec<i32> = Vec::new();
        while tmp_pos != pos {
            end_vec.push(game[tmp_pos]);
            tmp_pos = wrap_id(tmp_pos+1);
        }
        assert_eq!(end_vec.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""), result_1);
    }
}
