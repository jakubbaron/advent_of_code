use std::collections::HashSet;
use std::io::{self};

fn sums_to(packages: &Vec<usize>, sought_sum: usize, reduce_size: usize) -> Vec<Vec<usize>> {
    let mut combinations: Vec<Vec<usize>> = Vec::new();
    for i in 0..=1 << packages.len() / reduce_size {
        let mut t = i;
        let mut s = 0;
        let mut tmp_vec: Vec<usize> = Vec::new();
        for package in packages.iter() {
            if t % 2 == 1 {
                s += *package;
                tmp_vec.push(*package);
                if s >= sought_sum {
                    break;
                }
            }
            t /= 2;
        }
        if s == sought_sum {
            combinations.push(tmp_vec);
        }
    }
    combinations
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 1, 1),
        ("input.txt", 1, 1)
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let packages: Vec<usize> = file_content.into_iter().rev().map(|x| x.parse::<usize>().unwrap()).collect();
        let total_sum = packages.iter().fold(0, |acc, val| acc+val);
        println!("Total_sum {}", total_sum);
        let sought_sum = total_sum / 3;
        let vectors = sums_to(&packages, sought_sum, 2);
        println!("Sums len: {}", vectors.len());
        let mut pairs: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
        for v in vectors.iter() {
            let mut tmp:HashSet<usize> = packages.clone().into_iter().collect();
            for el in v.iter() {
                tmp.remove(el);
            }
            let tmp: Vec<usize> = tmp.into_iter().collect();
            for i in sums_to(&tmp, sought_sum, 1) {
                pairs.push((v.clone(), i));
            }
        }
        println!("Pairs len: {}", pairs.len());
        let mut triples: Vec<Vec<Vec<usize>>> = Vec::new();
        for (p, v) in pairs.into_iter() {
            let mut tmp:HashSet<usize> = packages.clone().into_iter().collect();
            for el in v.iter() {
                tmp.remove(el);
            }
            for el in p.iter() {
                tmp.remove(el);
            }
            let tmp: Vec<usize> = tmp.into_iter().collect();
            if tmp.iter().fold(0, |acc, val| acc + val) == sought_sum {
                triples.push(vec![p, v, tmp]);
            }
        }
        for el in triples.iter_mut() {
            el.sort_by(|a, b| a.len().cmp(&b.len()));
        }
        println!("{:?}", triples.len());
        let mut entanglement = (usize::MAX, usize::MAX);
        for v in triples.iter() {
            let g1 = &v[0];
            let product = g1.iter().fold(1, |acc, val| acc * val);
            if g1.len() < entanglement.0 {
                entanglement = (g1.len(), product);
            } else if g1.len() == entanglement.0 && product < entanglement.1 {
                entanglement = (g1.len(), product);
            }
        }
        println!("{:?}", entanglement);

    }
    Ok(())
}
