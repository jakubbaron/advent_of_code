use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self};

struct Rule {
    str_value: String,
    rule: Option<String>,
}

impl Rule {
    fn to_regex(&mut self, rules: &HashMap<i32, RefCell<Rule>>) -> String {
        match &self.rule {
            Some(rule) => {
                return rule.to_string();
            }
            None => (),
        };
        // ADD RULE BUILD
        if self.str_value.len() == 1 {
            let re = Regex::new("^[a-zA-Z]$").unwrap();
            if !re.is_match(&self.str_value) {
                println!("NOPE {}", self.str_value);
            }
            self.rule = Some(self.str_value.to_string());
            return self.str_value.to_string();
        }
        let groups: Vec<String> = self
            .str_value
            .split("|")
            .map(|x| x.trim().to_string())
            .collect();
        let mut full_rule: String = "(".to_owned();
        for (i, group) in groups.iter().enumerate() {
            let other_rules: Vec<i32> = group
                .split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            let mut s: String = "".to_owned();
            for r in other_rules.iter() {
                let tmp = rules.get(r).unwrap().borrow_mut().to_regex(&rules);
                s.push_str(&tmp);
            }
            if i != groups.len() - 1 {
                s.push_str(&"|");
            }
            full_rule.push_str(&s);
        }
        full_rule.push_str(&")");
        self.rule = Some(full_rule.to_string());
        return full_rule.to_string();
    }
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 2, 1),
        ("test2.txt", 3, 12),
        ("input.txt", 200, 407),
    ];
    for (f, result_1, result_2) in files_results.iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut rules: HashMap<i32, RefCell<Rule>> = HashMap::with_capacity(file_content.len());
        let mut start = 0;
        for (i, line) in file_content.iter().enumerate() {
            let rule_line: Vec<&str> = line.split(":").collect();
            if rule_line.len() == 1 {
                start = i;
                break;
            }
            let rule_no = rule_line[0].parse::<i32>().unwrap();
            rules.insert(
                rule_no,
                RefCell::new(Rule {
                    str_value: rule_line[1].trim().to_string().replace("\"", ""),
                    rule: None,
                }),
            );
        }

        let rule_zero = format!("^{}$", rules.get(&0).unwrap().borrow_mut().to_regex(&rules));
        let regex_zero = Regex::new(&rule_zero).unwrap();
        let mut sum = 0;
        for line in file_content[start + 1..file_content.len()].iter() {
            if regex_zero.is_match(&line) {
                sum += 1;
            }
        }
        assert_eq!(sum, *result_1);

        if !rules.contains_key(&42) {
            println!("{} didn't have a rule 42, continuing", f);
            continue;
        };

        if !rules.contains_key(&31) {
            println!("{} didn't have a rule 31, continuing", f);
            continue;
        };

        let rule_42 = format!("{}", rules.get(&42).unwrap().borrow_mut().to_regex(&rules));
        let rule_31 = format!("{}", rules.get(&31).unwrap().borrow_mut().to_regex(&rules));
        rules.insert(
            8,
            RefCell::new(Rule {
                str_value: "42 | 42 8".to_string(),
                rule: Some(format!("({}+)", rule_42)),
            }),
        );
        let mut tmp: String = format!("({}{})?", rule_42, rule_31);
        for _ in 0..20 {
            tmp = format!("({}{}{})?", rule_42, tmp, rule_31);
        }
        tmp = format!("({}{}{})", rule_42, tmp, rule_31);

        rules.insert(
            11,
            RefCell::new(Rule {
                str_value: "42 31 | 42 11 31".to_string(),
                rule: Some(format!("({}{}|{})", rule_42, rule_31, tmp)),
            }),
        );
        rules.insert(
            0,
            RefCell::new(Rule {
                str_value: "8 11".to_string(),
                rule: None,
            }),
        );

        let rule_zero = format!("^{}$", rules.get(&0).unwrap().borrow_mut().to_regex(&rules));
        let regex_zero = Regex::new(&rule_zero).unwrap();

        let mut sum = 0;
        for line in file_content[start + 1..file_content.len()].iter() {
            if regex_zero.is_match(&line) {
                sum += 1;
            }
        }
        assert_eq!(sum, *result_2);

        let regex_31 = Regex::new(&format!("^{}", rule_31)).unwrap();
        let regex_42 = Regex::new(&format!("^{}", rule_42)).unwrap();

        let mut sum = 0;
        for line in file_content[start + 1..file_content.len()].iter() {
            if !regex_42.is_match(&line) {
                continue;
            }
            let mut counter_42 = 0;
            let mut rest = line.to_string();
            while regex_42.is_match(&rest) {
                counter_42 += 1;
                let caps = regex_42.captures(&rest).unwrap();
                rest = rest.replacen(&caps.get(0).map_or("", |m| m.as_str()).to_string(), "", 1);
            }

            let mut counter_31 = 0;
            while regex_31.is_match(&rest) {
                counter_31 += 1;
                let caps = regex_31.captures(&rest).unwrap();
                rest = rest.replacen(&caps.get(0).map_or("", |m| m.as_str()).to_string(), "", 1);
            }
            if counter_42 > counter_31 && counter_31 > 0 && rest.is_empty() {
                sum += 1;
            }
        }
        assert_eq!(sum, *result_2);
    }

    Ok(())
}
