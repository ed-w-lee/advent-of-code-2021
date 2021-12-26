use std::path::Path;

use itertools::Itertools;

use crate::util::read_lines;

pub fn solution_1<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    let mut lines = read_lines(filename).expect("failed to read input");
    let line = lines.next().expect("no line").expect("no line 2");
    let positions: Vec<i32> = line
        .split_terminator(',')
        .map(|pos| pos.parse::<i32>().unwrap())
        .sorted()
        .collect();
    let opt_pos = positions[positions.len() / 2];
    positions.into_iter().map(|pos| (pos - opt_pos).abs()).sum()
}

pub fn solution_2<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    let mut lines = read_lines(filename).expect("failed to read input");
    let line = lines.next().expect("no line").expect("no line 2");
    let positions: Vec<i32> = line
        .split_terminator(',')
        .map(|pos| pos.parse::<i32>().unwrap())
        .sorted()
        .collect();
    let first_pos = *positions.first().unwrap();
    let last_pos = *positions.last().unwrap();
    (first_pos..last_pos)
        .map(|pot_pos| {
            positions
                .iter()
                .map(|pos| {
                    let dist = (*pos - pot_pos).abs();
                    dist * (dist + 1) / 2
                })
                .sum::<i32>()
        })
        .min()
        .unwrap()
}
