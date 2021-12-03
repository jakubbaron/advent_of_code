use std::io::{self};

fn vec_to_dec(container: &Vec<i32>) -> i32 {
    let helper: Vec<String> = container.iter().map(|x| x.to_string()).collect();
    let s: String = helper.into_iter().collect();
    i32::from_str_radix(&s, 2).unwrap()
}

fn most_significant(container: &Vec<i32>) -> Vec<i32> {
    let mut helper = container.clone();
    for i in 0..helper.len() {
        if helper[i] >= 0 {
            helper[i] = 1;
        } else {
            helper[i] = 0;
        }
    }
    helper
}

fn least_significant(container: &Vec<i32>) -> Vec<i32> {
    let mut helper = container.clone();
    for i in 0..helper.len() {
        if helper[i] >= 0 {
            helper[i] = 0;
        } else {
            helper[i] = 1;
        }
    }
    helper
}

fn count_collection(file_content: &Vec<String>) -> Vec<i32> {
    let mut helper: Vec<i32> = vec![0; file_content[0].len()];
    for line in file_content.iter() {
        for (i, ch) in line.chars().enumerate() {
            if ch == '1' {
                helper[i] += 1;
            } else {
                helper[i] -= 1;
            }
        }
    }
    helper
}

fn part_1(file_content: &Vec<String>) -> i32 {
    let helper = count_collection(&file_content);
    let gamma = vec_to_dec(&most_significant(&helper));
    let epsilon = vec_to_dec(&least_significant(&helper));
    gamma * epsilon
}

fn narrow_down_string_bits(pool: &Vec<String>, method: fn(&Vec<i32>) -> Vec<i32>) -> i32 {
    let mut current_pool = pool.clone();
    let length = pool[0].len();
    for ch_idx in 0..length {
        let tmp = method(&count_collection(&current_pool));
        let mut tmp_pool: Vec<String> = vec![];
        let ch = tmp[ch_idx].to_string();
        for line in current_pool.iter() {
            let vec_line: Vec<String> = line.chars().map(|x| x.to_string()).collect();
            if vec_line[ch_idx] == ch {
                tmp_pool.push(line.clone());
            }
        }
        current_pool = tmp_pool;
        if current_pool.len() == 1 {
            break;
        }
    }
    vec_to_dec(
        &current_pool[0]
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect(),
    )
}

fn part_2(file_content: &Vec<String>) -> i32 {
    let oxygen = narrow_down_string_bits(&file_content, most_significant);
    let co2 = narrow_down_string_bits(&file_content, least_significant);
    oxygen * co2
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 198, 230), ("input.txt", 4103154, 4245351)];
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
