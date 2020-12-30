use std::io::{self};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 25, 4, 2, 3), // 2 is from part 1, min number of bottles
        ("input.txt", 150, 654, 4, 57) // 4 is from part 1, min number of bottles
    ];
    for (f, capacity, result_1, no_of_bottles, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let containers: Vec<usize> = file_content.into_iter().map(|x| x.parse::<usize>().unwrap()).collect();
        let mut res_1 = 0;
        let mut m: HashMap<usize, usize> = HashMap::new();
        for i in 0..1 << containers.len() {
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

        fn counts(required: usize, containers: &[usize]) -> usize {
            if required == 0 {
                return 1;
            }
            if containers.len() == 0 {
                return 0;
            }
            let first = *&containers[0];
            let rest = &containers[1..];
            if first > required {
                counts(required, &rest)
            } else {
                counts(required - first, &rest) + counts(required, &rest)
            }
        }

        assert_eq!(counts(capacity, &containers), result_1);
        fn counts_2(required: usize, containers: &[usize], count: usize) -> usize {
            if required == 0 {
                return 1;
            }
            if containers.len() == 0 {
                return 0;
            }
            let first = *&containers[0];
            let rest = &containers[1..];
            if first > required || count == 0  {
                counts_2(required, &rest, count)
            } else {
                counts_2(required - first, &rest, count - 1) + counts_2(required, &rest, count)
            }
        }
        assert_eq!(counts_2(capacity, &containers, no_of_bottles), result_2);
    }
    Ok(())
}
