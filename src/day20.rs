use std::{collections::HashSet, path::Path};

use itertools::Itertools;

use crate::util::read_lines;

type Coord = (i64, i64);

struct Board {
    enhancement_algo: HashSet<usize>,
    foreground_lights: HashSet<Coord>,
    foreground_row_bound: (i64, i64),
    foreground_col_bound: (i64, i64),
    is_background_lit: bool,
}

impl Board {
    fn is_coord_lit(&self, coord: &Coord) -> bool {
        let (row, col) = coord;
        if row < &self.foreground_row_bound.0
            || row > &self.foreground_row_bound.1
            || col < &self.foreground_col_bound.0
            || col > &self.foreground_col_bound.1
        {
            // out of bounds
            self.is_background_lit
        } else {
            self.foreground_lights.contains(coord)
        }
    }

    fn step(&mut self) {
        let (row_min, row_max) = self.foreground_row_bound;
        let (col_min, col_max) = self.foreground_col_bound;
        let new_foreground: HashSet<Coord> = ((row_min - 1)..(row_max + 2))
            .cartesian_product((col_min - 1)..(col_max + 2))
            .filter_map(|(row, col)| {
                let binary = (-1..2).cartesian_product(-1..2).fold(
                    String::from(""),
                    |mut acc, (row_off, col_off)| {
                        if self.is_coord_lit(&(row + row_off, col + col_off)) {
                            acc.push('1');
                        } else {
                            acc.push('0');
                        }
                        acc
                    },
                );
                if self
                    .enhancement_algo
                    .contains(&usize::from_str_radix(&binary, 2).unwrap())
                {
                    Some((row, col))
                } else {
                    None
                }
            })
            .collect();
        self.foreground_lights = new_foreground;
        if self.enhancement_algo.contains(&0) {
            self.is_background_lit = !self.is_background_lit;
        }
        self.foreground_col_bound = (col_min - 1, col_max + 1);
        self.foreground_row_bound = (row_min - 1, row_max + 1);
    }
}

fn get_board<P>(filename: P) -> Board
where
    P: AsRef<Path>,
{
    let mut lines = read_lines(filename).expect("failed to read input");
    let enhancement_algo: HashSet<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .char_indices()
        .filter_map(|(i, c)| match c {
            '#' => Some(i),
            '.' => None,
            _ => unreachable!(),
        })
        .collect();
    let _ = lines.next();
    let lights: HashSet<Coord> = lines
        .into_iter()
        .enumerate()
        .map(|(row_idx, line)| {
            line.unwrap()
                .char_indices()
                .filter_map(|(col_idx, c)| {
                    if c == '#' {
                        Some((row_idx as i64, col_idx as i64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let row_max = lights.iter().map(|(row, _)| *row).max().unwrap();
    let col_max = lights.iter().map(|(_, col)| *col).max().unwrap();

    Board {
        enhancement_algo,
        foreground_lights: lights,
        foreground_row_bound: (0, row_max),
        foreground_col_bound: (0, col_max),
        is_background_lit: false,
    }
}

pub fn solution_1<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let mut board = get_board(filename);
    board.step();
    board.step();
    board.foreground_lights.len()
}

pub fn solution_2<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let mut board = get_board(filename);
    for _ in 0..50 {
        board.step();
    }
    board.foreground_lights.len()
}
