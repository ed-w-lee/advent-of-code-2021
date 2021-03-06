use std::{collections::HashMap, path::Path};

use itertools::Itertools;

use crate::util::read_lines;

pub fn solution_1<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename).expect("failed to read input");
    lines
        .into_iter()
        .map(|line| {
            line.expect("couldn't find line")
                .split_terminator(" -> ")
                .map(|coord| {
                    coord
                        .split_terminator(',')
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32)>()
                })
                .map(|coord| coord.unwrap())
                .collect_tuple::<((i32, i32), (i32, i32))>()
        })
        .map(|coord_pair| coord_pair.unwrap())
        .filter_map(|((lx, ly), (rx, ry))| {
            if lx != rx && ly != ry {
                None
            } else if lx == rx {
                let (miny, maxy) = if ly < ry { (ly, ry) } else { (ry, ly) };
                Some(
                    (miny..(maxy + 1))
                        .map(|y| (lx, y))
                        .collect::<Vec<(i32, i32)>>(),
                )
            } else {
                let (minx, maxx) = if lx < rx { (lx, rx) } else { (rx, lx) };
                Some(
                    (minx..(maxx + 1))
                        .map(|x| (x, ly))
                        .collect::<Vec<(i32, i32)>>(),
                )
            }
        })
        .flatten()
        .fold(HashMap::<(i32, i32), i32>::new(), |mut counts, coord| {
            *(counts.entry(coord).or_insert(0)) += 1;
            counts
        })
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .count()
}

pub fn solution_2<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename).expect("failed to read input");
    lines
        .into_iter()
        .map(|line| {
            line.expect("couldn't find line")
                .split_terminator(" -> ")
                .map(|coord| {
                    coord
                        .split_terminator(',')
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32)>()
                })
                .map(|coord| coord.unwrap())
                .collect_tuple::<((i32, i32), (i32, i32))>()
        })
        .map(|coord_pair| coord_pair.unwrap())
        .map(|((lx, ly), (rx, ry))| {
            if lx != rx && ly != ry {
                let itx = if lx < rx { 1 } else { -1 };
                let ity = if ly < ry { 1 } else { -1 };
                let num_points = if lx < rx { rx - lx } else { lx - rx };
                (0..(num_points + 1))
                    .map(|i| (lx + i * itx, ly + i * ity))
                    .collect::<Vec<(i32, i32)>>()
            } else if lx == rx {
                let (miny, maxy) = if ly < ry { (ly, ry) } else { (ry, ly) };
                (miny..(maxy + 1))
                    .map(|y| (lx, y))
                    .collect::<Vec<(i32, i32)>>()
            } else {
                let (minx, maxx) = if lx < rx { (lx, rx) } else { (rx, lx) };
                (minx..(maxx + 1))
                    .map(|x| (x, ly))
                    .collect::<Vec<(i32, i32)>>()
            }
        })
        .flatten()
        .fold(HashMap::<(i32, i32), i32>::new(), |mut counts, coord| {
            *(counts.entry(coord).or_insert(0)) += 1;
            counts
        })
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .count()
}
