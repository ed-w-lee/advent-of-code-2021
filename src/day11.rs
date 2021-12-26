use std::{collections::{HashMap, HashSet}, path::Path};

use crate::util::read_lines;

const SURROUND: [(i32, i32); 8] = [
    (1, 1),
    (0, 1),
    (-1, 1),
    (1, 0),
    (-1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn step(grid: &mut HashMap<(i32, i32), u32>) -> usize {
    let mut has_flashed: HashSet<(i32, i32)> = HashSet::new();
    grid.clone().keys().into_iter().for_each(|coord| {
        *(grid.get_mut(coord).unwrap()) += 1;
    });
    loop {
        let new_flashes: Vec<(i32, i32)> = grid
            .iter()
            .filter_map(|(coord, level)| {
                if *level > 9 && !has_flashed.contains(coord) {
                    Some(*coord)
                } else {
                    None
                }
            })
            .collect();
        if new_flashes.is_empty() {
            break;
        }
        has_flashed.extend(new_flashes.iter());

        new_flashes.iter().for_each(|(x, y)| {
            SURROUND.iter().for_each(|(offx, offy)| {
                if let Some(entry) = grid.get_mut(&(x + offx, y + offy)) {
                    *entry += 1;
                }
            })
        })
    }
    grid.clone().into_iter().for_each(|(coord, level)| {
        *(grid.get_mut(&coord).unwrap()) = if level > 9 { 0 } else { level }
    });
    has_flashed.len()
}

pub fn solution_1<P>(filename: P, steps: u32) -> usize where P: AsRef<Path> {
    let lines = read_lines(filename).expect("failed to read input");

    let mut vals: HashMap<(i32, i32), u32> = lines
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

    (0..steps).fold(0usize, |flashes, _| flashes + step(&mut vals))
}

pub fn solution_2<P>(filename: P) -> usize where P: AsRef<Path> {
    let lines = read_lines(filename).expect("failed to read input");

    let mut vals: HashMap<(i32, i32), u32> = lines
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

    let mut my_step = 0;
    loop {
        my_step += 1;
        if step(&mut vals) == vals.len() {
            break;
        }
    }
    my_step
}
