use std::collections::HashSet;
use std::io::{self};

fn game_of_life(mut lights: Vec<Vec<bool>>, steps: usize, keep_corners: bool) -> usize {
    let neighbours: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let corners: HashSet<(usize, usize)> = vec![
        (0, 0),
        (0, lights[0].len() - 1),
        (lights.len() - 1, 0),
        (lights.len() - 1, lights[lights.len() - 1].len() - 1),
    ]
    .into_iter()
    .collect();
    for _s in 0..steps {
        let mut tmp = lights.to_vec();
        for i in 0..lights.len() {
            for j in 0..lights[i].len() {
                let mut counter = 0;
                for (ii, jj) in neighbours.iter() {
                    let iii = i as i32 + *ii;
                    let jjj = j as i32 + *jj;
                    if iii < 0 {
                        continue;
                    }
                    if jjj < 0 {
                        continue;
                    }
                    let iii = iii as usize;
                    let jjj = jjj as usize;
                    if iii >= lights.len() {
                        continue;
                    }
                    if jjj >= lights[iii].len() {
                        continue;
                    }
                    if lights[iii][jjj] {
                        counter += 1;
                    }
                }
                if corners.contains(&(i, j)) && keep_corners {
                    tmp[i][j] = true;
                } else if lights[i][j] && (counter == 2 || counter == 3) {
                    tmp[i][j] = true;
                } else if !lights[i][j] && counter == 3 {
                    tmp[i][j] = true;
                } else {
                    tmp[i][j] = false;
                }
            }
        }
        lights = tmp;
    }
    lights
        .into_iter()
        .flat_map(|row| row.into_iter())
        .fold(0, |acc, val| acc + val as usize)
}
fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 4, 4, 17, 5), ("input.txt", 821, 100, 886, 100)];
    for (f, result_1, steps_1, result_2, steps_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let lights: Vec<Vec<bool>> = file_content
            .into_iter()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        assert_eq!(game_of_life(lights.to_vec(), steps_1, false), result_1);
        let mut copy_lights = lights.to_vec();
        let len = copy_lights.len();
        let len_row = copy_lights[0].len();
        copy_lights[0][0] = true;
        copy_lights[0][len_row - 1] = true;
        copy_lights[len - 1][0] = true;
        copy_lights[len - 1][len_row - 1] = true;

        assert_eq!(game_of_life(copy_lights, steps_2, true), result_2);
    }
    Ok(())
}
