use regex::Regex;
use std::collections::HashMap;
use std::io::{self};

trait Mask {
    fn apply_mask(&self, number: u64) -> u64;
}

struct MaskV1 {
    mask: u64,
    set: bool,
}

impl Mask for MaskV1 {
    fn apply_mask(&self, number: u64) -> u64 {
        if self.set {
            return number | self.mask;
        } else {
            return number & !self.mask;
        }
    }
}

impl Mask for MaskV2 {
    fn apply_mask(&self, number: u64) -> u64 {
        if self.set {
            return number | self.mask;
        }
        number
    }
}

struct MaskV2 {
    mask: u64,
    set: bool,
    float: bool,
}

enum MaskKind {
    MaskV1,
    MaskV2,
}

impl MaskV2 {
    fn to_masks_v1(&self) -> (MaskV1, MaskV1) {
        (
            MaskV1 {
                mask: self.mask,
                set: self.set,
            },
            MaskV1 {
                mask: self.mask,
                set: !self.set,
            },
        )
    }
}

fn parse_masks_v1(mask: &str) -> Vec<MaskV1> {
    let mut result: Vec<MaskV1> = Vec::new();
    for (i, ch) in mask.chars().rev().enumerate() {
        if ch == 'X' {
            continue;
        }
        let set = ch == '1';
        result.push(MaskV1 { mask: 1 << i, set });
    }
    result
}

fn parse_masks_v2(mask: &str) -> Vec<MaskV2> {
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

fn apply_masks_v1(number: u64, masks: &Vec<MaskV1>) -> u64 {
    masks.iter().fold(number, |acc, mask| mask.apply_mask(acc))
}

fn apply_masks_v2(number: u64, masks: &Vec<MaskV2>) -> u64 {
    masks.iter().fold(number, |acc, mask| mask.apply_mask(acc))
}

fn generate_addresses(number: u64, masks: &Vec<MaskV2>) -> Vec<u64> {
    let mut floats: Vec<u64> = vec![apply_masks_v2(number, masks)];
    for iter_mask in masks.iter() {
        if !iter_mask.float {
            continue;
        }
        let (m1, m2) = iter_mask.to_masks_v1();
        let mut tmp = Vec::new();
        for n in floats {
            tmp.push(m1.apply_mask(n));
            tmp.push(m2.apply_mask(n));
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
        let mut mask = Vec::new();
        let mut maskv2 = Vec::new();
        for line in vec.iter() {
            if re_mask.is_match(&line) {
                let caps = re_mask.captures(&line).unwrap();
                let tmp = caps.get(1).map_or("", |m| m.as_str());
                mask = parse_masks_v1(&tmp);
                maskv2 = parse_masks_v2(&tmp);
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

            mem_values_1.insert(mem_address, apply_masks_v1(value, &mask));

            for mem_addr_2 in generate_addresses(mem_address, &maskv2) {
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
