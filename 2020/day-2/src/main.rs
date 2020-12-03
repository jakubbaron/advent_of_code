use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

struct Password {
    character: String,
    min_occur: usize,
    max_occur: usize,
    password: String,
}

impl Password {
    fn from_string(input: &str) -> Password {
        let splitted = input.split(" ");
        let vec: Vec<&str> = splitted.collect();
        let occur: Vec<&str> = vec[0].split("-").collect();
        let min_occur = occur[0].parse::<usize>().unwrap();
        let max_occur = occur[1].parse::<usize>().unwrap();
        let character = String::from(&vec[1][0..1]);
        let password = String::from(&vec[2][..]);
        Password {
            character,
            min_occur,
            max_occur,
            password,
        }
    }

    fn is_valid(self) -> bool {
        let c = self.password.matches(&self.character).count();
        return c >= self.min_occur && c <= self.max_occur;
    }
}
fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut counter = 0;
    for line in f.lines() {
        let my_string = line.unwrap();
        let pass = Password::from_string(&my_string);
        if pass.is_valid() {
            counter += 1;
        }
    }
    println!("{}", counter);

    Ok(())
}
