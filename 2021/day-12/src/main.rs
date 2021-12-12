use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self};

fn get_graph(file_content: &Vec<String>) -> HashMap<String, VecDeque<String>> {
    let mut output: HashMap<String, VecDeque<String>> = HashMap::new();
    for line in file_content.iter() {
        let splitted: Vec<_> = line.split("-").collect::<Vec<_>>();
        let start = splitted[0];
        let end = splitted[1];
        let entry = output
            .entry(start.to_string())
            .or_insert(VecDeque::from(vec![]));
        entry.push_back(end.to_string());
        let entry = output
            .entry(end.to_string())
            .or_insert(VecDeque::from(vec![]));
        entry.push_back(start.to_string());
    }
    output.remove("end");
    for (_k, v) in output.iter_mut() {
        if let Some(index) = v.iter().position(|x| *x == "start") {
            v.remove(index);
        }
    }
    output
}

fn is_lowercase(edge: &String) -> bool {
    edge.chars().all(|c| c.is_ascii_lowercase())
}

fn is_small_cave_visited_twice(current_path: &Vec<String>) -> bool {
    let mut duplicates: HashSet<String> = HashSet::new();
    for entry in current_path.iter().filter(|x| is_lowercase(x)) {
        if duplicates.contains(entry) {
            return true;
        }
        duplicates.insert(entry.clone());
    }
    false
}

fn more_than_one_small_cave_visited_twice(current_path: &Vec<String>) -> bool {
    let mut duplicates: HashMap<String, usize> = HashMap::new();
    for entry in current_path.iter().filter(|x| is_lowercase(x)) {
        let entry = duplicates.entry(entry.to_string()).or_insert(0);
        *entry += 1;
    }
    let mut double_counts = 0;
    for value in duplicates.values() {
        if *value > 2 {
            return true;
        }
        if *value == 2 {
            double_counts += 1;
        }
        if double_counts == 2 {
            return true;
        }
    }
    false
}

fn find_all_paths(
    graph: &HashMap<String, VecDeque<String>>,
    start: &String,
    end: &String,
    path: &Vec<String>,
    method: fn(&Vec<String>) -> bool,
) -> Vec<Vec<String>> {
    let mut current_path = path.clone();
    current_path.push(start.to_string());
    if method(&current_path) {
        return vec![];
    }
    if start == end {
        return vec![current_path.clone()];
    }
    if !graph.contains_key(start) {
        return vec![];
    }

    let mut paths: Vec<Vec<String>> = vec![];
    for node in graph[start].iter() {
        let new_paths = find_all_paths(&graph, node, end, &current_path, method);
        for p in new_paths {
            paths.push(p)
        }
    }
    paths
}

fn part_1(file_content: &Vec<String>) -> usize {
    let graph = get_graph(&file_content);
    let empty_path = vec![];
    let paths = find_all_paths(
        &graph,
        &String::from("start"),
        &String::from("end"),
        &empty_path,
        is_small_cave_visited_twice,
    );
    paths.len()
}

fn part_2(file_content: &Vec<String>) -> usize {
    let graph = get_graph(&file_content);
    let empty_path = vec![];
    let paths = find_all_paths(
        &graph,
        &String::from("start"),
        &String::from("end"),
        &empty_path,
        more_than_one_small_cave_visited_twice,
    );
    paths.len()
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
