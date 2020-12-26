use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self};
// use std::ops::{BitAnd, BitOr};
use regex::Regex;

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 72, 1),
        ("input.txt", 3176, 1),
        ("input2.txt", 14710, 1),
    ];
    for (f, result_1, _result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut wires: VecDeque<(&str, &str)> = VecDeque::with_capacity(file_content.len());
        for line in file_content.iter() {
            let splitted: Vec<&str> = line.split(" -> ").collect();
            assert_eq!(splitted.len(), 2);
            wires.push_back((&splitted[0], &splitted[1]));
        }
        let mut signals: HashMap<&str, u16> = HashMap::new();
        let re_and_or = Regex::new(r"(.*) (AND|OR) (.*)").unwrap();
        let re_rl_shift = Regex::new(r"(.*) (RSHIFT|LSHIFT) (\d+)").unwrap();
        let re_not = Regex::new(r"NOT (.*)").unwrap();
        while let Some(current) = wires.pop_front() {
            let (signal, wire) = current;
            match signal.parse::<u16>() {
                Ok(s) => {
                    signals.insert(wire, s);
                    continue;
                }
                Err(_) => (),
            }
            if re_and_or.is_match(&signal) {
                let caps = re_and_or.captures(&signal).unwrap();
                let wire_1 = caps.get(1).map_or("", |x| x.as_str());
                let wire_2 = caps.get(3).map_or("", |x| x.as_str());
                let gate = caps.get(2).map_or("", |x| x.as_str());
                let signal_1 = match signals.get(&wire_1) {
                    Some(s) => *s,
                    None => match wire_1.parse::<u16>() {
                        Ok(s) => s,
                        Err(_) => {
                            wires.push_back((signal, wire));
                            continue;
                        }
                    },
                };
                let signal_2 = match signals.get(&wire_2) {
                    Some(s) => *s,
                    None => match wire_2.parse::<u16>() {
                        Ok(s) => s,
                        Err(_) => {
                            wires.push_back((signal, wire));
                            continue;
                        }
                    },
                };
                let s = if gate == "AND" {
                    signal_1 & signal_2
                } else {
                    signal_1 | signal_2
                };
                signals.insert(wire, s);
                continue;
            } else if re_rl_shift.is_match(&signal) {
                let caps = re_rl_shift.captures(&signal).unwrap();
                let wire_1 = caps.get(1).map_or("", |x| x.as_str());
                let signal_1 = match signals.get(&wire_1) {
                    Some(s) => *s,
                    None => {
                        wires.push_back((signal, wire));
                        continue;
                    }
                };
                let shift_val = caps
                    .get(3)
                    .map_or("", |x| x.as_str())
                    .parse::<u16>()
                    .unwrap();
                let shift = caps.get(2).map_or("", |x| x.as_str());

                let s = if shift == "RSHIFT" {
                    signal_1 >> shift_val
                } else {
                    signal_1 << shift_val
                };
                signals.insert(wire, s);
                continue;
            } else if re_not.is_match(&signal) {
                let caps = re_not.captures(&signal).unwrap();
                let wire_1 = caps.get(1).map_or("", |x| x.as_str());
                let signal_1 = match signals.get(&wire_1) {
                    Some(s) => *s,
                    None => {
                        wires.push_back((signal, wire));
                        continue;
                    }
                };
                let s = !signal_1;
                signals.insert(wire, s);
                continue;
            } else {
                let wire_1 = signal;
                let signal_1 = match signals.get(&wire_1) {
                    Some(s) => *s,
                    None => {
                        wires.push_back((signal, wire));
                        continue;
                    }
                };
                signals.insert(wire, signal_1);
            }
        }
        let signal_a = *signals.get(&"a").unwrap();
        assert_eq!(signal_a, result_1);
    }
    Ok(())
}
