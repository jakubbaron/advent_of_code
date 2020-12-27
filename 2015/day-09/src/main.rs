use std::collections::HashSet;
use std::io::{self};

#[derive(Debug, Clone, Copy)]
struct CityId(usize);

#[derive(Debug, Clone, Copy)]
struct Distance(f64);

fn get_vertex_with_min_distance(
    min_distances: &Vec<f64>,
    visited: &HashSet<usize>,
) -> (usize, f64) {
    let mut current_min_distance = f64::MAX;
    let mut vertex = usize::MAX;

    for (vertex_idx, distance) in min_distances.iter().enumerate() {
        if visited.contains(&vertex_idx) {
            continue;
        }
        if distance <= &current_min_distance {
            vertex = vertex_idx;
            current_min_distance = *distance
        }
    }
    (vertex, current_min_distance)
}

fn dijkstra_algorithm(start: usize, edges: &Vec<Vec<(CityId, Distance)>>) -> Vec<f64> {
    let number_of_vertices = edges.len();
    let mut min_distances = vec![f64::MAX; number_of_vertices];
    min_distances[start] = 0.0;
    let mut visited: HashSet<usize> = HashSet::new();
    while visited.len() != number_of_vertices {
        let (vertex, current_min_distance) = get_vertex_with_min_distance(&min_distances, &visited);
        if current_min_distance == f64::MAX {
            break;
        }
        visited.insert(vertex);

        for edge in edges[vertex].iter() {
            let (destination, distance_to_destination) = edge;
            if visited.contains(&destination.0) {
                continue;
            }
            let new_path_distance = current_min_distance + distance_to_destination.0;
            if min_distances[destination.0] > new_path_distance {
                min_distances[destination.0] = new_path_distance;
            }
        }
    }
    min_distances
}

fn main() -> io::Result<()> {
    let files_results = vec![
        ("test.txt", 605_f64, 982_f64),
        ("input.txt", 117_f64, 909_f64),
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut cities: Vec<&str> = Vec::new();
        let mut distances: Vec<Vec<(CityId, Distance)>> = Vec::new();
        for line in file_content.iter() {
            let cities_value: Vec<&str> = line.split(" = ").collect();
            let city_names: Vec<&str> = cities_value[0].split(" to ").collect();
            let value = cities_value[1].parse::<f64>().unwrap();
            let city_1 = city_names[0];
            let city_2 = city_names[1];
            let id_1 = if let Some(i) = (0..cities.len()).find(|&i| cities[i] == city_1) {
                i
            } else {
                cities.push(city_1);
                distances.push(Vec::new());
                cities.len() - 1
            };
            let id_2 = if let Some(i) = (0..cities.len()).find(|&i| cities[i] == city_2) {
                i
            } else {
                cities.push(city_2);
                distances.push(Vec::new());
                cities.len() - 1
            };
            distances[id_1].push((CityId(id_2), Distance(value)));
            distances[id_2].push((CityId(id_1), Distance(value)));
        }
        println!("{:?}", cities);
        for row in distances.iter() {
            println!("{:?}", row);
        }
        let mut total_distance = 0_f64;
        let mut visited: HashSet<usize> = HashSet::new();
        let mut current = 0;
        while !visited.len() != cities.len() {
            visited.insert(current);
            let current_dijkstra = dijkstra_algorithm(current, &distances);
            let mut dijkstra_with_indices: Vec<(f64, usize)> = current_dijkstra
                .iter()
                .enumerate()
                .filter(|(i, _)| !visited.contains(i))
                .map(|(x, y)| (*y, x))
                .collect();
            dijkstra_with_indices.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            let (val, min_id) = if let Some((val, min_id)) = dijkstra_with_indices.first() {
                (val, min_id)
            } else {
                break;
            };
            println!("Dijkstra: {:?}", current_dijkstra);
            println!("MinId {}, Val {}", min_id, val);
            current = *min_id;
            total_distance += current_dijkstra[current];
        }
        println!("Total distance {}", total_distance);
        assert_eq!(total_distance, result_1);

        let mut inverted_distances = vec![Vec::new(); distances.len()];
        for (i, edges) in distances.iter().enumerate() {
            for edge in edges.iter() {
                let (destination, distance_to_destination) = edge;
                inverted_distances[i]
                    .push((*destination, Distance(1.0 / distance_to_destination.0)));
            }
        }
        let mut max_total_distance = 0_f64;
        for i in 0..distances.len() {
            let mut current = i;
            let mut total_distance = 0_f64;
            let mut visited: HashSet<usize> = HashSet::new();
            while !visited.len() != cities.len() {
                visited.insert(current);
                let current_dijkstra = dijkstra_algorithm(current, &inverted_distances);
                let mut dijkstra_with_indices: Vec<(f64, usize)> = current_dijkstra
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !visited.contains(i))
                    .map(|(x, y)| (*y, x))
                    .collect();
                dijkstra_with_indices.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                let (val, max_id) = if let Some((val, max_id)) = dijkstra_with_indices.first() {
                    (val, max_id)
                } else {
                    break;
                };
                println!("Dijkstra: {:?}", current_dijkstra);
                println!("maxId {}, Val {}", max_id, val);
                current = *max_id;
                total_distance += 1.0 / current_dijkstra[current];
            }
            if total_distance > max_total_distance {
                max_total_distance = total_distance;
            }
            println!("Total distance {} for {}", total_distance, i);
        }
        println!("Max total distance {}", max_total_distance);
        assert_eq!(max_total_distance, result_2);
    }
    Ok(())
}
