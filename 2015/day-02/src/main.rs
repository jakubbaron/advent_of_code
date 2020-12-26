use std::io::{self};

fn get_dimensions(line: &str) -> (usize, usize, usize) {
    let dimensions: Vec<usize> = line.split("x").map(|x| x.parse::<usize>().unwrap()).collect();
    assert_eq!(dimensions.len(), 3);
    (dimensions[0], dimensions[1], dimensions[2])
}

fn get_result_1(file_content: &Vec<String>) -> usize {
    let mut all_areas = 0;
    for line in file_content.iter() {
        let (l, w, h) = get_dimensions(&line);
        let sizes = vec![l*w, l*h, w*h];
        let smallest = *sizes.iter().min().unwrap();
        all_areas += sizes.into_iter().fold(smallest, |acc, val| acc + 2 * val);
    }
    all_areas
}

fn get_result_2(file_content: &Vec<String>) -> usize {
    let mut all_ribbon = 0;
    for line in file_content.iter() {
        let (l, w, h) = get_dimensions(&line);
        let side_sizes = vec![2*l + 2*w, 2*l + 2*h, 2*w + 2*h];
        let smallest = *side_sizes.iter().min().unwrap();
        let volume = l * w * h;
        all_ribbon += smallest + volume;
    }
    all_ribbon
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 58, 34),
        ("test2.txt", 43, 14),
        ("input.txt", 1588178, 3783758),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(get_result_1(&file_content), result_1);
        assert_eq!(get_result_2(&file_content), result_2);
    }
    Ok(())
}
