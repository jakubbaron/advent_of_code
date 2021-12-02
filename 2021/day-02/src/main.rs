use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 150, 900),
        (
            "input.txt",
            1250395,
            1451210346,
        ),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut depth = 0;
        let mut position = 0;
        for line in file_content.iter() {
            let instruction: Vec<&str> = line.split(" ").collect();
            let command = instruction[0];
            let number = instruction[1].parse::<i32>().unwrap();
            match command {
                "forward" => {
                    position += number
                }
                "up" => {
                    depth -= number
                }
                "down" => {
                    depth += number
                }
                _ => (),
            }
        }
        let res_1 = position * depth;
        assert_eq!(res_1, result_1);

        let mut depth = 0;
        let mut position = 0;
        let mut aim = 0;
        for line in file_content.iter() {
            let instruction: Vec<&str> = line.split(" ").collect();
            let command = instruction[0];
            let number = instruction[1].parse::<i32>().unwrap();
            match command {
                "forward" => {
                    position += number;
                    depth += aim * number;
                }
                "up" => {
                    aim -= number;
                }
                "down" => {
                    aim += number;
                }
                _ => (),
            }
        }

        let res_2 = position * depth;
        assert_eq!(res_2, result_2);
    }
    Ok(())
}
