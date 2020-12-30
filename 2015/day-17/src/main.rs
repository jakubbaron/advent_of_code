use std::io::{self};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 25, 4, 3),
        ("input.txt", 150, 654, 57)
    ];
    for (f, capacity, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let containers: Vec<usize> = file_content.into_iter().map(|x| x.parse::<usize>().unwrap()).collect();
        let mut res_1 = 0;
        let mut m: HashMap<usize, usize> = HashMap::new();
        for i in 0..1<<containers.len() {
            let mut t = i;
            let mut s = 0;
            let mut p = 0;
            for container in containers.iter() {
                if t % 2 == 1 {
                    s += *container;
                    p += 1;
                }
                t /= 2;
            }
            if s == capacity {
                res_1 += 1;
                m.entry(p).and_modify(|x| *x+=1).or_insert(1);
            }
        }
        assert_eq!(res_1, result_1);
        let min_key = m.keys().min().unwrap();
        assert_eq!(*m.get(min_key).unwrap(), result_2);
    }
    Ok(())
}
