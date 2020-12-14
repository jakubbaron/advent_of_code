use regex::Regex;
use std::collections::HashMap;
use std::io::{self};

#[derive(Debug, Clone)]
struct Mask {
    mask: u64,
    set: bool,
}

struct MaskV2 {
    mask: u64,
    set: bool,
    float: bool,
}

fn parse_mask(mask: &str) -> Vec<Mask> {
    let mut result: Vec<Mask> = Vec::new();
    for (i, ch) in mask.chars().rev().enumerate() {
        if ch == 'X' {
            continue;
        }
        let set = ch == '1';
        result.push(Mask { mask: 1 << i, set });
    }
    result
}

fn parse_mask_v2(mask: &str) -> Vec<MaskV2> {
    let mut result: Vec<MaskV2> = Vec::new();
    for (i, ch) in mask.chars().rev().enumerate() {
        let float = ch == 'X';
        let set = ch == '1';
        result.push(MaskV2 {
            mask: 1 << i,
            set,
            float,
        });
    }
    result
}

fn apply_single_mask(number: u64, mask: &Mask) -> u64 {
    if mask.set {
        return number | mask.mask;
    } else {
        return number & !mask.mask;
    }
}

fn apply_masks_1(number: u64, masks: &Vec<Mask>) -> u64 {
    let mut tmp = number;
    for mask in masks.iter() {
        tmp = apply_single_mask(tmp, mask);
    }
    tmp
}

fn apply_single_mask_v2(number: u64, masks: &Vec<MaskV2>) -> u64 {
    let mut tmp = number;
    for mask in masks.iter() {
        if mask.float {
            continue;
        }
        if mask.set {
            tmp = tmp | mask.mask;
        }
    }
    tmp
}

fn apply_masks_2(number: u64, masks: &Vec<MaskV2>) -> Vec<u64> {
    let mut floats: Vec<u64> = vec![apply_single_mask_v2(number, masks)];
    for iter_mask in masks.iter() {
        if !iter_mask.float {
            continue;
        }
        let MaskV2 {
            float: _,
            mask,
            set: _,
        } = iter_mask;
        let mut tmp = Vec::new();
        for n in floats {
            tmp.push(apply_single_mask(
                n,
                &Mask {
                    mask: *mask,
                    set: true,
                },
            ));
            tmp.push(apply_single_mask(
                n,
                &Mask {
                    mask: *mask,
                    set: false,
                },
            ));
        }
        floats = tmp.to_vec();
    }
    floats
}

fn main() -> io::Result<()> {
    let files_results = vec![
        // ("test.txt", 165, 1068781_u64), // this takes forever with a lot of floating X
        ("test2.txt", 51, 208),
        ("input.txt", 13727901897109, 5579916171823),
    ];
    let re_mask = Regex::new(r"^mask = (\S+)$").unwrap();
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();

    for (f, result_1, result_2) in files_results.iter() {
        println!("{}", f);
        let vec: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut mem_values_1: HashMap<u64, u64> = HashMap::new();
        let mut mem_values_2: HashMap<u64, u64> = HashMap::new();
        let mut mask = parse_mask(&"X".repeat(36));
        let mut maskv2 = parse_mask_v2(&"0".repeat(36));
        for line in vec.iter() {
            if re_mask.is_match(&line) {
                let caps = re_mask.captures(&line).unwrap();
                let tmp = caps.get(1).map_or("", |m| m.as_str());
                mask = parse_mask(&tmp);
                maskv2 = parse_mask_v2(&tmp);
                continue;
            }
            if !re_mem.is_match(&line) {
                println!("NOPE {}", &line);
            }
            let caps = re_mem.captures(&line).unwrap();
            let mem_address = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<u64>()
                .unwrap();
            let value = caps
                .get(2)
                .map_or("0", |m| m.as_str())
                .parse::<u64>()
                .unwrap();
            let value_after_mask = apply_masks_1(value, &mask);
            mem_values_1.insert(mem_address, value_after_mask);
            for mem_addr_2 in apply_masks_2(mem_address, &maskv2) {
                mem_values_2.insert(mem_addr_2, value);
            }
        }
        let mut part_1 = 0;
        for (_k, vals) in mem_values_1.iter() {
            part_1 += vals;
        }
        let mut part_2 = 0;
        for (_k, vals) in mem_values_2.iter() {
            part_2 += vals;
        }
        assert_eq!(part_1, *result_1);
        assert_eq!(part_2, *result_2);
    }

    Ok(())
}
