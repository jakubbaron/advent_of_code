use std::io::{self};

fn count_bigger_neighbours(container: &Vec<u32>) -> usize {
    let mut result = 0;
    for i in 1..container.len() {
        if container[i] > container[i - 1] {
            result += 1;
        }
    }
    result
}

fn running_sum(container: &Vec<u32>, sum_size: usize) -> Vec<u32> {
    let sum_1 = sum_size - 1;
    let mut output: Vec<u32> = vec![0; container.len() - sum_1];
    for i in sum_1..container.len() {
        for j in 0..sum_size {
            output[i - sum_1] += container[i - j]
        }
    }
    output
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 7, 5), ("input.txt", 1288, 1311)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<u32> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect();

        let res_1 = count_bigger_neighbours(&file_content);
        assert_eq!(res_1, result_1);

        let helper: Vec<u32> = running_sum(&file_content, 3);
        let res_2 = count_bigger_neighbours(&helper);

        assert_eq!(res_2, result_2);
    }
    Ok(())
}
