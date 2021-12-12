use std::collections::{HashMap, HashSet};
use std::io::{self};

fn get_graph(file_content: &Vec<String>) -> HashMap<&str, Vec<&str>> {
    let mut output: HashMap<_, _> = HashMap::new();
    for line in file_content.iter() {
        let splitted: Vec<_> = line.split("-").collect::<Vec<_>>();
        let start = splitted[0];
        let end = splitted[1];
        let entry = output.entry(start).or_insert(vec![]);
        entry.push(end);
        let entry = output.entry(end).or_insert(vec![]);
        entry.push(start);
    }
    output.remove("end");
    for (_k, v) in output.iter_mut() {
        if let Some(index) = v.iter().position(|x| *x == "start") {
            v.remove(index);
        }
    }
    output
}

fn is_lowercase(edge: &str) -> bool {
    edge.chars().all(|c| c.is_ascii_lowercase())
}

fn is_small_cave_visited_twice(current_path: &Vec<&str>) -> bool {
    let mut duplicates: HashSet<_> = HashSet::new();
    for entry in current_path.iter().filter(|x| is_lowercase(x)) {
        if duplicates.contains(entry) {
            return true;
        }
        duplicates.insert(entry);
    }
    false
}

fn more_than_one_small_cave_visited_twice(current_path: &Vec<&str>) -> bool {
    let mut duplicates: HashMap<_, _> = HashMap::new();
    for entry in current_path.iter().filter(|x| is_lowercase(x)) {
        let entry = duplicates.entry(entry).or_insert(0);
        *entry += 1;
        if *entry > 2 {
            return true;
        }
    }
    duplicates.values().filter(|&x| *x == 2).count() > 1
}

fn count_all_paths(
    graph: &HashMap<&str, Vec<&str>>,
    start: &str,
    end: &str,
    path: &Vec<&str>,
    method: fn(&Vec<&str>) -> bool,
) -> usize {
    if start == end {
        return 1;
    }

    let mut current_path = path.to_vec();
    current_path.push(start);
    if method(&current_path) {
        return 0;
    }

    graph[start].iter().fold(0, |acc, node| {
        acc + count_all_paths(&graph, node, end, &current_path, method)
    })
}

fn part_1(file_content: &Vec<String>) -> usize {
    count_all_paths(
        &get_graph(&file_content),
        "start",
        "end",
        &Vec::new(),
        is_small_cave_visited_twice,
    )
}

fn part_2(file_content: &Vec<String>) -> usize {
    count_all_paths(
        &get_graph(&file_content),
        "start",
        "end",
        &Vec::new(),
        more_than_one_small_cave_visited_twice,
    )
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 10, 36),
        ("test1.txt", 19, 103),
        ("test2.txt", 226, 3509),
        ("input.txt", 3450, 96528),
    ];
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
