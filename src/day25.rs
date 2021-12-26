use std::{collections::HashSet, path::Path, cmp::max};

use itertools::Itertools;

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

fn step(max_row: usize, max_col: usize, east_facing: &mut HashSet<(usize, usize)>, south_facing: &mut HashSet<(usize, usize)>) -> bool {
    let mut changed = false;

    let next_east_facing: HashSet<_> = east_facing.iter().cloned().map(|(r, c)| {
        let next_pos = (r, (c + 1) % max_col);
        if east_facing.contains(&next_pos) || south_facing.contains(&next_pos) {
            (r, c)
        } else {
            changed = true;
            next_pos
        }
    }).collect();
    east_facing.clone_from(&next_east_facing);

    let next_south_facing: HashSet<_> = south_facing.iter().cloned().map(|(r, c)| {
        let next_pos = ((r + 1) % max_row, c);
        if east_facing.contains(&next_pos) || south_facing.contains(&next_pos) {
            (r, c)
        } else {
            changed = true;
            next_pos
        }
    }).collect();
    south_facing.clone_from(&next_south_facing);

    changed
}

pub fn solution_1<P>(filename: P) -> usize where P: AsRef<Path> {
    let lines = read_lines(filename).expect("failed to read input");

    let mut max_row = 0;
    let mut max_col = 0;
    let grid_vals: Vec<_> = lines
        .into_iter()
        .enumerate()
        .map(|(row, line)| {
            max_row = max(max_row, row + 1);
            line.unwrap()
                .chars()
                .enumerate()
                .map(|(col, c)| {
                    max_col = max(max_col, col + 1);
                    ((row, col), c)
                })
                .collect::<Vec<_>>()
        })
        .flatten().collect_vec();
    let mut east_facing: HashSet<_> = grid_vals.iter().filter_map(|(coord, c)| {
        match c {
            '>' => Some(*coord),
            _ => None,
        }
    }).collect();
    let mut south_facing: HashSet<_> = grid_vals.iter().filter_map(|(coord, c)| {
        match c {
            'v' => Some(*coord),
            _ => None,
        }
    }).collect();

    for i in 1.. {
        if !step(max_row, max_col, &mut east_facing, &mut south_facing) {
            return i;
        }
    }
    unreachable!()
}
