use day_20::{check_monsters, match_neighbours, Frame, create_picture, print_picture, frames_to_picture};
use std::cell::RefCell;
use std::collections::{HashMap};
use std::io::{self};

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
        match_neighbours(&frames);

        let res_1 = frames
            .values()
            .map(|x| x.borrow())
            .filter(|x| x.get_neighbours_len() == 2)
            .fold(1, |acc, frame| acc * frame.frame_no);
        assert_eq!(res_1, *result_1);

        let end_vec = create_picture(&frames);
        print_picture(&end_vec);

        let end_data = frames_to_picture(end_vec);
        let monster: Vec<Vec<char>> = std::fs::read_to_string("monster.txt")?
            .lines()
            .map(|x| x.chars().collect())
            .collect();
        let res_2 = check_monsters(&end_data, &monster);
        assert_eq!(res_2, *result_2);
    }
    Ok(())
}
