use std::io::{self};

fn find_loop_size(key: usize, subject: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;
    while value != key {
        if loop_size != 0 && loop_size % 100_000 == 0 {
            println!("Loop size: {}, subject {}", loop_size, subject);
        }
        loop_size += 1;
        value = (value * subject) % 20201227;
    }
    loop_size
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 14897079, 1), ("input.txt", 290487, 1)];
    for (f, result_1, _result_2) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let card_key = file_content[0].parse::<usize>().unwrap();
        let door_key = file_content[1].parse::<usize>().unwrap();
        println!("Card key: {}", card_key);
        println!("Door key: {}", door_key);

        let mut encryption_key = 1;
        let subject = 7;
        for _ in 0..find_loop_size(card_key, subject) {
            encryption_key = (encryption_key * door_key) % 20201227;
        }
        println!("Encryption key {}", encryption_key);
        assert_eq!(encryption_key, result_1)
    }
    Ok(())
}
