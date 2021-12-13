use crate::util::read_lines;
use std::{collections::{HashMap, HashSet}, path::Path};

fn is_small(s: &str) -> bool {
    s.chars().nth(0).unwrap().is_lowercase()
}

fn count_paths(
    edge_map: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    curr: &str,
) -> usize {
    // println!("{:?}", curr);
    // println!("{:?}", visited);
    if curr == "end" {
        return 1;
    }
    if visited.contains(curr) {
        return 0;
    }
    if is_small(curr) {
        visited.insert(curr.to_string());
    }
    let mut count = 0;
    for neighbor in edge_map.get(curr).unwrap() {
        count += count_paths(edge_map, visited, neighbor);
    }
    visited.remove(curr);
    count
}

pub fn solution_1<P>(filename: P) -> usize where P: AsRef<Path> {
    let lines = read_lines(filename).expect("failed to read input");
    let edges: Vec<(String, String)> = lines
        .into_iter()
        .map(|line| {
            let unwrapped = line.unwrap();
            let mut edge_str = unwrapped.split('-');
            let (a, b) = (edge_str.next().unwrap(), edge_str.next().unwrap());
            [
                (a.to_string(), b.to_string()),
                (b.to_string(), a.to_string()),
            ]
        })
        .flatten()
        .collect();
    let mut edge_map: HashMap<String, Vec<String>> = HashMap::new();
    for (a, b) in edges {
        (edge_map.entry(a.to_string()).or_insert(Vec::new())).push(b.to_string());
    }
    let mut visited: HashSet<String> = HashSet::new();
    count_paths(&edge_map, &mut visited, "start")
}

fn count_paths_double(
    edge_map: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    can_revisit: bool,
    curr: &str,
) -> usize {
    if curr == "end" {
        return 1;
    }
    let mut can_revisit_new = can_revisit;
    let mut hit_limit = false;
    if visited.contains(curr) {
        if can_revisit {
            can_revisit_new = false;
            hit_limit = true;
        } else {
            return 0;
        }
    }
    if is_small(curr) {
        visited.insert(curr.to_string());
    }
    let mut count = 0;
    for neighbor in edge_map.get(curr).unwrap() {
        if neighbor == "start" {
            continue;
        }
        count += count_paths_double(edge_map, visited, can_revisit_new, neighbor);
    }
    if !hit_limit {
        visited.remove(curr);
    }
    count
}

pub fn solution_2<P>(filename: P) -> usize where P: AsRef<Path> {
    let lines = read_lines(filename).expect("failed to read input");
    let edges: Vec<(String, String)> = lines
        .into_iter()
        .map(|line| {
            let unwrapped = line.unwrap();
            let mut edge_str = unwrapped.split('-');
            let (a, b) = (edge_str.next().unwrap(), edge_str.next().unwrap());
            [
                (a.to_string(), b.to_string()),
                (b.to_string(), a.to_string()),
            ]
        })
        .flatten()
        .collect();
    let mut edge_map: HashMap<String, Vec<String>> = HashMap::new();
    for (a, b) in edges {
        (edge_map.entry(a.to_string()).or_insert(Vec::new())).push(b.to_string());
    }
    let mut visited: HashSet<String> = HashSet::new();
    count_paths_double(&edge_map, &mut visited, true, "start")
}
