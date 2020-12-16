use std::io::{self};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Range {
    lower_min: usize,
    lower_max: usize,
    upper_min: usize,
    upper_max: usize,
}
impl Range {
    fn in_range(&self, number: usize) -> bool {
        (self.lower_min <= number && self.lower_max >= number) || (self.upper_min <= number && self.upper_max >= number)
    }

    fn from_vec(numbers: &Vec<usize>) -> Range {
        let mut tmp_vec = numbers.to_vec();
        tmp_vec.sort();
        assert_eq!(numbers.len(), 4);
        assert_eq!(&tmp_vec, numbers);

        Range {
            lower_min: numbers[0],
            lower_max: numbers[1],
            upper_min: numbers[2],
            upper_max: numbers[3],
        }
    }
}
fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 71, 1068781_u64),
        // ("test2.txt", 51, 208),
        ("input.txt", 18227, 5579916171823),
    ];
    for (f, result_1, result_2) in files_results.iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let position_your_ticket = file_content.iter().position(|x| x.as_str() == "your ticket:").unwrap();

        let re_field = Regex::new(r"^([a-zA-Z0-9_ ]*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        let mut fields: HashMap<String, Range> = HashMap::new();
        for field_line in file_content[0..position_your_ticket-1].iter() {
            if !re_field.is_match(&field_line) {
                println!("NOPE {}", field_line);
                continue;
            }
            let caps = re_field.captures(&field_line).unwrap();
            let name = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let numbers: Vec<usize> = (2..=5).map(|x| caps.get(x).map_or("", |m| m.as_str()).parse::<usize>().unwrap()).collect();
            fields.insert(name, Range::from_vec(&numbers));
        }

        let mut invalid_fields_sum = 0;
        let position_nearby_tickets = file_content.iter().position(|x| x.as_str() == "nearby tickets:").unwrap();
        let mut valid_tickets:Vec<Vec<usize>> = Vec::new();
        for ticket_line in file_content[position_nearby_tickets+1..file_content.len()].iter() {
            let ticket_fields:Vec<usize> = ticket_line.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
            let mut is_valid_ticket = true;
            for field_value in ticket_fields.iter() {
                let mut is_valid_field = false;
                for (k, range) in fields.iter() {
                    if range.in_range(*field_value) {
                        is_valid_field = true;
                        break;
                    }
                }
                if !is_valid_field {
                    invalid_fields_sum += field_value;
                    is_valid_ticket = false;
                }
            }
            if is_valid_ticket {
                valid_tickets.push(ticket_fields);
            }
        }
        // let my_ticket = &file_content[position_your_ticket + 1];
        println!("Part 1, invalid fields sum: {}", invalid_fields_sum);
        assert_eq!(invalid_fields_sum, *result_1);
    }
    Ok(())
}
