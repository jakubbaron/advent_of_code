use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self};

#[derive(Debug, Clone)]
struct Range {
    lower_min: usize,
    lower_max: usize,
    upper_min: usize,
    upper_max: usize,
}

impl Range {
    fn in_range(&self, number: usize) -> bool {
        (self.lower_min <= number && self.lower_max >= number)
            || (self.upper_min <= number && self.upper_max >= number)
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
        ("test.txt", 71, 1),
        ("test2.txt", 0, 1),
        ("input.txt", 18227, 2355350878831),
    ];
    for (f, result_1, result_2) in files_results.iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let position_your_ticket = file_content
            .iter()
            .position(|x| x.as_str() == "your ticket:")
            .unwrap();

        let re_field = Regex::new(r"^([a-zA-Z ]*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        let mut fields: HashMap<String, Range> = HashMap::new();
        for field_line in file_content[0..position_your_ticket - 1].iter() {
            if !re_field.is_match(&field_line) {
                println!("NOPE {}", field_line);
                continue;
            }
            let caps = re_field.captures(&field_line).unwrap();
            let name = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let numbers: Vec<usize> = (2..=5)
                .map(|x| {
                    caps.get(x)
                        .map_or("", |m| m.as_str())
                        .parse::<usize>()
                        .unwrap()
                })
                .collect();
            fields.insert(name, Range::from_vec(&numbers));
        }

        let mut invalid_fields_sum = 0;
        let position_nearby_tickets = file_content
            .iter()
            .position(|x| x.as_str() == "nearby tickets:")
            .unwrap();
        let mut valid_tickets: Vec<Vec<usize>> = Vec::new();
        for ticket_line in file_content[position_nearby_tickets + 1..file_content.len()].iter() {
            let ticket_fields: Vec<usize> = ticket_line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let mut is_valid_ticket = true;
            for field_value in ticket_fields.iter() {
                let mut is_valid_field = false;
                for (_, range) in fields.iter() {
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
        println!("Part 1, invalid fields sum: {}", invalid_fields_sum);
        assert_eq!(invalid_fields_sum, *result_1);

        let no_cols = valid_tickets[0].len();
        assert!(valid_tickets.iter().all(|row| row.len() == no_cols));

        let mut columns: Vec<Vec<usize>> = Vec::new();
        for col in 0..no_cols {
            columns.push(valid_tickets.iter().map(|row| row[col]).collect());
        }

        let keys: Vec<String> = fields.keys().cloned().collect();
        let mut maybe_keys: Vec<Vec<String>> = Vec::new();
        for column in columns.iter() {
            let mut tmp: Vec<String> = Vec::new();
            for key in keys.iter() {
                let range = fields.get(key).unwrap();
                if column.iter().all(|x| range.in_range(*x)) {
                    tmp.push(key.to_string());
                }
            }
            maybe_keys.push(tmp);
        }

        let mut sorted_keys: Vec<(usize, Vec<String>)> =
            maybe_keys.into_iter().enumerate().collect();
        sorted_keys.sort_by_key(|(_k, val)| val.len());

        let mut seen: HashSet<String> = HashSet::new();
        let mut valid_keys: Vec<String> = vec!["".to_string(); columns.len()];
        for (col_id, keys) in sorted_keys.iter() {
            for key in keys.iter() {
                if !seen.contains(key) {
                    seen.insert(key.to_string());
                    valid_keys[*col_id] = key.to_string();
                }
            }
        }
        println!("{:?}", valid_keys);

        let my_ticket: Vec<usize> = file_content[position_your_ticket + 1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let product = valid_keys
            .iter()
            .zip(my_ticket.iter())
            .filter(|(key, _val)| key.contains("departure"))
            .fold(1, |acc, (_, val)| val * acc);

        println!("Product: {}", product);
        assert_eq!(product, *result_2)
    }

    Ok(())
}
