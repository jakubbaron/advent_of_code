use regex::Regex;
use std::collections::HashMap;
use std::io::{self};

trait MaskTrait {
    fn apply_mask(&self, number: u64) -> u64;
}

struct MaskV1 {
    mask: u64,
    set: bool,
}

impl MaskTrait for MaskV1 {
    fn apply_mask(&self, number: u64) -> u64 {
        if self.set {
            number | self.mask
        } else {
            number & !self.mask
        }
    }
}

impl MaskTrait for MaskV2 {
    fn apply_mask(&self, number: u64) -> u64 {
        if self.set {
            number | self.mask
        } else {
            number
        }
    }
}

impl MaskV2 {
    fn to_masks_v1(&self) -> (MaskV1, MaskV1) {
        let MaskV2 {
            mask,
            set,
            float: _,
        } = *self;
        (MaskV1 { mask, set }, MaskV1 { mask, set: !set })
    }
}

struct MaskV2 {
    mask: u64,
    set: bool,
    float: bool,
}

fn parse_masks_v1(mask: &str) -> Vec<MaskV1> {
    mask.chars()
        .rev()
        .enumerate()
        .filter(|(_i, ch)| *ch != 'X')
        .map(|(i, ch)| MaskV1 {
            mask: 1 << i,
            set: ch == '1',
        })
        .collect()
}

fn parse_masks_v2(mask: &str) -> Vec<MaskV2> {
    mask.chars()
        .rev()
        .enumerate()
        .map(|(i, ch)| MaskV2 {
            mask: 1 << i,
            set: ch == '1',
            float: ch == 'X',
        })
        .collect()
}

fn apply_masks<T: MaskTrait>(number: u64, masks: &Vec<T>) -> u64 {
    masks.iter().fold(number, |acc, mask| mask.apply_mask(acc))
}

fn generate_addresses(number: u64, masks: &Vec<MaskV2>) -> Vec<u64> {
    masks
        .iter()
        .filter(|x| x.float)
        .map(|x| x.to_masks_v1())
        .fold(vec![apply_masks(number, masks)], |acc, (m1, m2)| {
            acc.into_iter()
                .flat_map(|n| vec![m1.apply_mask(n), m2.apply_mask(n)].into_iter())
                .collect()
        })
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
        let mut masks_v1 = Vec::new();
        let mut masks_v2 = Vec::new();
        for line in vec.iter() {
            if re_mask.is_match(&line) {
                let caps = re_mask.captures(&line).unwrap();
                let tmp = caps.get(1).map_or("", |m| m.as_str());
                masks_v1 = parse_masks_v1(&tmp);
                masks_v2 = parse_masks_v2(&tmp);
                continue;
            }
            if !re_mem.is_match(&line) {
                println!("NOPE {}", &line);
                assert!(false);
            }
            let caps = re_mem.captures(&line).unwrap();
            let mem_address = caps
                .get(1)
                .map_or("", |m| m.as_str())
                .parse::<u64>()
                .unwrap();
            let value = caps
                .get(2)
                .map_or("", |m| m.as_str())
                .parse::<u64>()
                .unwrap();

            mem_values_1.insert(mem_address, apply_masks(value, &masks_v1));

            for mem_addr_2 in generate_addresses(mem_address, &masks_v2) {
                mem_values_2.insert(mem_addr_2, value);
            }
        }

        assert_eq!(
            mem_values_1.iter().fold(0, |acc, (_, val)| acc + val),
            *result_1
        );
        assert_eq!(
            mem_values_2.iter().fold(0, |acc, (_, val)| acc + val),
            *result_2
        );
    }

    Ok(())
}
