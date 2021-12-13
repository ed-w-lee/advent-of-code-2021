use std::{collections::{HashMap, HashSet, VecDeque}, path::Path};

use itertools::Itertools;

use crate::util::read_lines;

pub fn solution_1<P>(filename: P) -> u32 where P: AsRef<Path> {
    let lines = read_lines(filename).expect("failed to read input");
    let vals: HashMap<(i32, i32), u32> = lines
        .into_iter()
        .enumerate()
        .map(|(row, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(col, c)| ((row as i32, col as i32), c.to_digit(10).unwrap()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    const DIFFS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    vals.iter()
        .filter_map(|((x, y), height)| {
            if DIFFS
                .iter()
                .all(|(offx, offy)| match vals.get(&(x + offx, y + offy)) {
                    None => true,
                    Some(neighbor_height) => neighbor_height > height,
                })
            {
                Some(height)
            } else {
                None
            }
        })
        .map(|height| *height + 1)
        .sum()
}

pub fn solution_2<P>(filename: P) -> usize where P: AsRef<Path> {
    let lines = read_lines(filename).expect("failed to read input");
    let vals: HashMap<(i32, i32), u32> = lines
        .into_iter()
        .enumerate()
        .map(|(row, line)| {
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(col, c)| ((row as i32, col as i32), c.to_digit(10).unwrap()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    const DIFFS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let low_points: Vec<(i32, i32)> = vals
        .iter()
        .filter_map(|((x, y), height)| {
            if DIFFS
                .iter()
                .all(|(offx, offy)| match vals.get(&(x + offx, y + offy)) {
                    None => true,
                    Some(neighbor_height) => neighbor_height > height,
                })
            {
                Some((x.clone(), y.clone()))
            } else {
                None
            }
        })
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut basins: HashMap<(i32, i32), usize> =
        low_points.iter().map(|point| (point.clone(), 0)).collect();
    for low_point in low_points {
        let mut queue: VecDeque<(i32, i32)> = VecDeque::from([low_point.clone()]);
        // println!("BOUNDARY");
        while let Some(next_point) = queue.pop_front() {
            // println!("{:?}", next_point);
            // bfs
            if visited.contains(&next_point) {
                continue;
            }
            if let Some(height) = vals.get(&next_point) {
                if *height < 9 {
                    *(basins.entry(low_point).or_insert(0)) += 1;
                } else {
                    continue;
                }
            } else {
                continue;
            }
            visited.insert(next_point.clone());

            let (x, y) = next_point;
            DIFFS.iter().for_each(|(offx, offy)| {
                queue.push_back((x + offx, y + offy));
            });
        }
    }
    println!("{:?}", basins);
    let sizes: Vec<usize> = basins.drain().map(|(_, v)| v).sorted().rev().collect();
    sizes[0] * sizes[1] * sizes[2]
}
