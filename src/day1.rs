use crate::util::read_lines;
use std::{iter::IntoIterator, path::{Path}};

fn num_higher(depths: &Vec<i32>) -> usize {
    depths.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

fn num_higher_3_wide(depths: &Vec<i32>) -> usize {
    depths.windows(4).filter(|pair| pair[0] < pair[3]).count()
}

pub fn solution_1<P>(fname: P) -> usize where P: AsRef<Path> {
    let lines = read_lines(fname).expect("failed to open");
    let depths: Vec<i32> = lines
        .into_iter()
        .map(|f| f.expect("not a line").parse().expect("not a num"))
        .collect();
    num_higher(&depths)
}

pub fn solution_2<P>(fname: P) -> usize where P: AsRef<Path> {
    let lines = read_lines(fname).expect("failed to open");
    let depths: Vec<i32> = lines
        .into_iter()
        .map(|f| f.expect("not a line").parse().expect("not a num"))
        .collect();
    num_higher_3_wide(&depths)
}
