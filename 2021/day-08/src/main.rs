use std::collections::{HashMap, HashSet};
use std::io::{self};

fn parse_input(file_content: &Vec<String>) -> Vec<Vec<Vec<&str>>> {
    let mut output: Vec<Vec<Vec<&str>>> = vec![];
    for line in file_content.iter() {
        let splitted: Vec<&str> = line.split(" | ").collect();
        let input_digits: Vec<&str> = splitted[0].split(" ").collect();
        let displayed_digits: Vec<&str> = splitted[1].split(" ").collect();
        output.push(vec![input_digits, displayed_digits]);
    }
    output
}
fn part_1(file_content: &Vec<String>) -> i32 {
    let inputs = parse_input(&file_content);
    let mut counters: HashMap<i32, i32> = HashMap::new();
    for line in inputs.iter() {
        let displayed = &line[1];
        for display in displayed.iter() {
            let entry = counters.entry(display.len() as i32).or_insert(0);
            *entry += 1;
        }
    }
    let mut result = 0;
    for digit in vec![2, 3, 4, 7].iter() {
        result += counters[digit];
    }
    result
}

fn string_to_set(string: &String) -> HashSet<char> {
    string.chars().collect()
}

fn part_2(file_content: &Vec<String>) -> i32 {
    let digits_mapping: HashMap<i32, Vec<char>> = vec![
        (0, vec!['a', 'b', 'c', 'e', 'f', 'g']),
        (1, vec!['c', 'f']),
        (2, vec!['a', 'c', 'd', 'e', 'g']),
        (3, vec!['a', 'c', 'd', 'f', 'g']),
        (4, vec!['b', 'c', 'd', 'f']),
        (5, vec!['a', 'b', 'd', 'f', 'g']),
        (6, vec!['a', 'b', 'd', 'e', 'f', 'g']),
        (7, vec!['a', 'c', 'f']),
        (8, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        (9, vec!['a', 'b', 'c', 'd', 'f', 'g']),
    ]
    .into_iter()
    .collect();

    let mut string_to_digit: HashMap<String, String> = HashMap::new();
    for (k, v) in digits_mapping.iter() {
        string_to_digit.insert(v.iter().collect(), k.to_string());
    }
    let inputs = parse_input(&file_content);
    let mut output = 0;
    for line in inputs.iter() {
        let mut mapping: HashMap<char, char> = HashMap::new();
        let digits = &line[0];
        let displayed = &line[1];
        let mut copied: Vec<String> = digits.iter().map(|x| x.to_string()).collect();
        copied.sort_by_key(|x| x.len());
        let copied = copied;
        let eight = string_to_set(&copied[copied.len() - 1]);
        let one = string_to_set(&copied[0]);
        let seven = string_to_set(&copied[1]);
        let four = string_to_set(&copied[2]);
        let a_set = (&seven - &one)
            .iter()
            .map(|x| x.clone())
            .collect::<HashSet<char>>();
        let a = a_set.iter().collect::<Vec<_>>()[0].clone();
        mapping.insert(a, 'a');

        let five_lines = copied
            .iter()
            .filter(|x| x.len() == 5)
            .map(|x| string_to_set(x))
            .collect::<Vec<_>>();
        let mut middles: HashSet<char> = five_lines[0].clone();
        for l in five_lines.iter() {
            middles = middles.intersection(l).cloned().collect();
        }
        let d_set = middles
            .intersection(&four)
            .map(|x| x.clone())
            .collect::<HashSet<char>>();
        let d = d_set.iter().collect::<Vec<_>>()[0].clone();
        mapping.insert(d, 'd');

        let top_bottom = (&middles - &four)
            .iter()
            .map(|x| x.clone())
            .collect::<HashSet<char>>();
        let g_set = (&top_bottom - &seven)
            .iter()
            .map(|x| x.clone())
            .collect::<HashSet<char>>();
        let g = g_set.iter().collect::<Vec<_>>()[0].clone();
        mapping.insert(g, 'g');

        let six_lines = copied
            .iter()
            .filter(|x| x.len() == 6)
            .map(|x| string_to_set(x))
            .collect::<Vec<_>>();
        let mut c_set: HashSet<char>;
        let f_set: HashSet<char>;
        let mut f: char = 'f';
        let mut c: char = 'c';
        for maybe_six in six_lines.iter() {
            let maybe_c = (&eight - &maybe_six)
                .iter()
                .map(|x| x.clone())
                .collect::<HashSet<_>>();
            c_set = maybe_c
                .intersection(&one)
                .cloned()
                .collect::<HashSet<char>>();
            if c_set.len() == 1 {
                c = c_set.iter().collect::<Vec<_>>()[0].clone();
                mapping.insert(c, 'c');
                f_set = (&one - &c_set)
                    .iter()
                    .map(|x| x.clone())
                    .collect::<HashSet<char>>();
                f = f_set.iter().collect::<Vec<_>>()[0].clone();
                mapping.insert(f, 'f');
                break;
            }
        }

        let cdf: HashSet<char> = vec![c, d, f].iter().map(|x| x.clone()).collect();
        let b_set = (&four - &cdf)
            .iter()
            .map(|x| x.clone())
            .collect::<HashSet<char>>();
        let b = b_set.iter().collect::<Vec<_>>()[0].clone();
        mapping.insert(b, 'b');
        let seven_lines = copied
            .iter()
            .filter(|x| x.len() == 7)
            .map(|x| string_to_set(x))
            .collect::<Vec<_>>()[0]
            .clone();
        let e_helper_set = (&seven_lines - &four)
            .iter()
            .map(|x| x.clone())
            .collect::<HashSet<char>>();
        let e_set = (&e_helper_set - &top_bottom)
            .iter()
            .map(|x| x.clone())
            .collect::<HashSet<char>>();
        let e = e_set.iter().collect::<Vec<_>>()[0].clone();
        assert_eq!(e_set.len(), 1);
        mapping.insert(e, 'e');
        let mut keys = mapping.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        // println!("a_set {:?}", a_set);
        // println!("b_set {:?}", b_set);
        // println!("c_set {:?}", c_set);
        // println!("d_set {:?}", d_set);
        // println!("e_set {:?}", e_set);
        // println!("f_set {:?}", f_set);
        // println!("g_set {:?}", g_set);
        // for k in keys.iter() {
        //     println!("{} -> {}", k, mapping[k]);
        // }
        assert_eq!(mapping.len(), 7);

        let mut stringified_digits: Vec<String> = vec![];
        for digit in displayed.iter() {
            let mut reverse_mapping: Vec<char> = vec![];
            for ch in digit.chars() {
                reverse_mapping.push(mapping[&ch]);
            }
            reverse_mapping.sort();
            let str_mapping: String = reverse_mapping.iter().collect();
            stringified_digits.push(string_to_digit[&str_mapping].clone());
        }
        let str_number: String = stringified_digits.into_iter().collect();
        let number = str_number.parse::<i32>().unwrap();
        output += number;
    }
    output
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 26, 61229), ("input.txt", 445, 1043101)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let res_1 = part_1(&file_content);
        assert_eq!(res_1, result_1);

        let res_2 = part_2(&file_content);
        assert_eq!(res_2, result_2);
    }
    Ok(())
}
