use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use regex::Regex;

#[derive(Debug)]
#[derive(Clone)]
struct Bag {
    color: String,
    number: i32,
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    // let f = File::open("test.txt")?;
    let f = BufReader::new(f);
    let mut vec = Vec::new();
    let mut all_bags: HashMap<String, Vec<Bag>> = HashMap::new();

    for line in f.lines() {
        let my_string = line.unwrap();
        vec.push(my_string);
    }
    let vec = vec;

    for line in &vec {
        let field_values: Vec<&str> = line.split(" contain ").collect();
        let mut tmp = String::from(field_values[0]);
        let main_bag: String = tmp.drain(..tmp.find(" bags").unwrap()).collect();
        // println!("Main bag: {}", main_bag);
        let other_bags = field_values[1].split(",").collect::<Vec<&str>>();
        if other_bags[0] == "no other bags." {
            continue;
        }
        // println!("other_bags: {:?}", other_bags);

        let re = Regex::new(r"^(\d+) (.*)").unwrap();
        let mut tmp_bags: Vec<Bag> = Vec::new();
        for other_bag in other_bags.iter() {
            let mut tmp: String = other_bag.trim().to_string();
            let bag: String = tmp.drain(..tmp.find(" bag").unwrap()).collect();
            if !re.is_match(&bag) {
                println!("NOPE {}", &bag);
            }
            let caps = re.captures(&bag).unwrap();
            // println!("caps {:?}", caps);
            let number = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<i32>()
                .unwrap();
            let color = caps.get(2).map_or("", |m| m.as_str()).to_string();
            // println!("{} `{}`", number, color);
            tmp_bags.push(Bag { color, number });
        }

        all_bags.insert(main_bag, tmp_bags);
    }
    let sought_color = "shiny gold";
    let mut counter = 0;
    for (_k, v) in all_bags.iter() {
        let mut queue: Vec<Bag> = v.to_vec();
        while !queue.is_empty() {
            let item = queue.pop().unwrap();
            let Bag{ color, number: _ } = item;
            if color == sought_color {
                counter += 1;
                break;
            }
            match all_bags.get(&color) {
                Some(v) => queue.extend(v.to_vec()),
                None => (),
            }
        }
        queue.clear();
    }
    println!("Counter: {}", counter);
    Ok(())
}
