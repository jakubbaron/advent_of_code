use regex::Regex;
use std::io::{self};
use std::collections::HashMap;
use std::cell::RefCell;

//let re_field = Regex::new(r"^([a-zA-Z ]*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
// if !re_field.is_match(&field_line) {
//     println!("NOPE {}", field_line);
//     continue;
// }
// let caps = re_field.captures(&field_line).unwrap();
// let name = caps.get(1).map_or("", |m| m.as_str()).to_string();
// let numbers: Vec<usize> = (2..=5)
//     .map(|x| {
//         caps.get(x)
//             .map_or("", |m| m.as_str())
//             .parse::<usize>()
//             .unwrap()
//     })
//     .collect();

struct Rule {
    str_value: String,
    rule: Option<String>,
}

impl Rule {
    fn to_regex(&mut self, rules: &HashMap<i32, RefCell<Rule>>) -> String {
        match &self.rule {
            Some(rule) => {return rule.to_string();}
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
        let groups:Vec<String> = self.str_value.split("|").map(|x| x.trim().to_string()).collect();
        let mut full_rule: String = "(".to_owned();
        for (i, group) in groups.iter().enumerate() {
            let other_rules:Vec<i32> = group.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
            let mut s: String = "".to_owned();
            for r in other_rules.iter() {
                let tmp = rules.get(r).unwrap().borrow_mut().to_regex(&rules);
                s.push_str(&tmp);
            }
            if i != groups.len() -1 {
                s.push_str(&"|");
            }
            full_rule.push_str(&s);
        }
        full_rule.push_str(&")");
        self.rule = Some(full_rule.to_string());
        return full_rule.to_string();
    }
}
// let aabb = "^(aa|bb)$";
// let re = Regex::new(&aabb).unwrap();
// let test_str = "aa";
// let test_str_2 = "aaa";
// match re.is_match(&test_str) {
//     true => println!("YES"),
//     false => println!("NO"),
// }
// match re.is_match(&test_str_2) {
//     true => println!("YES"),
//     false => println!("NO"),
// }

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 2, 1),
        ("test2.txt", 3, 1),
        ("input.txt", 200, 1),
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
            let rule_line:Vec<&str> = line.split(":").collect();
            if rule_line.len() == 1 {
                start = i;
                break;
            }
            let rule_no = rule_line[0].parse::<i32>().unwrap();
            rules.insert(rule_no, RefCell::new(Rule{str_value: rule_line[1].trim().to_string().replace("\"", ""), rule: None}));
        }

        // for (k, rule) in rules.iter() {
        //     let mut p = rule.borrow_mut();
        //     println!("{} {}", k, p.to_regex(&rules));
        // }
        let rule_zero = format!("^{}$", rules.get(&0).unwrap().borrow_mut().to_regex(&rules));
        println!("{}", rule_zero);

        let regex_zero = Regex::new(&rule_zero).unwrap();
        let mut sum = 0;
        for line in file_content[start+1..file_content.len()].iter() {
            if regex_zero.is_match(&line) {
                sum += 1;
            }
        }
        assert_eq!(sum, *result_1);

        // let mut rules: HashMap<i32, RefCell<Rule>> = HashMap::with_capacity(file_content.len());
        // let mut start = 0;
        // for (i, line) in file_content.iter().enumerate() {
        //     let rule_line:Vec<&str> = line.split(":").collect();
        //     if rule_line.len() == 1 {
        //         start = i;
        //         break;
        //     }
        //     let rule_no = rule_line[0].parse::<i32>().unwrap();
        //     rules.insert(rule_no, RefCell::new(Rule{str_value: rule_line[1].trim().to_string().replace("\"", ""), rule: None}));
        // }
        // rules.insert(8, RefCell::new(Rule{str_value:


    }
    Ok(())
}
