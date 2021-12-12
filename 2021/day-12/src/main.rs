use std::collections::{HashMap, HashSet};
use std::io::{self};

fn get_graph(file_content: &Vec<String>) -> HashMap<&str, Vec<&str>> {
    let mut output: HashMap<_, _> = HashMap::new();
    for line in file_content.iter() {
        let splitted: Vec<_> = line.split("-").collect::<Vec<_>>();
        let start = splitted[0];
        let end = splitted[1];
        let entry = output
            .entry(start)
            .or_insert(vec![]);
        entry.push(end);
        let entry = output
            .entry(end)
            .or_insert(vec![]);
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
    let mut duplicates: HashSet<&str> = HashSet::new();
    for entry in current_path.iter().filter(|x| is_lowercase(x)) {
        if duplicates.contains(entry) {
            return true;
        }
        duplicates.insert(entry.clone());
    }
    false
}

fn more_than_one_small_cave_visited_twice(current_path: &Vec<&str>) -> bool {
    let mut duplicates: HashMap<&str, usize> = HashMap::new();
    for entry in current_path.iter().filter(|x| is_lowercase(x)) {
        let entry = duplicates.entry(entry).or_insert(0);
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

fn count_all_paths(
    graph: &HashMap<&str, Vec<&str>>,
    start: &str,
    end: &str,
    path: &Vec<&str>,
    method: fn(&Vec<&str>) -> bool,
) -> usize {
    let mut current_path = path.clone();
    current_path.push(start);
    if method(&current_path) {
        return 0;
    }
    if start == end {
        return 1;
    }
    if !graph.contains_key(start) {
        return 0;
    }

    let mut counter = 0;
    for node in graph[start].iter() {
        counter += count_all_paths(&graph, node, end, &current_path, method);
    }
    counter
}

fn part_1(file_content: &Vec<String>) -> usize {
    let graph = get_graph(&file_content);
    let empty_path = vec![];
    count_all_paths(
        &graph,
        "start",
        "end",
        &empty_path,
        is_small_cave_visited_twice,
    )
}

fn part_2(file_content: &Vec<String>) -> usize {
    let graph = get_graph(&file_content);
    let empty_path = vec![];
    count_all_paths(
        &graph,
        "start",
        "end",
        &empty_path,
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
