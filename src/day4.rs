use std::{cmp, collections::HashMap, path::Path};

use itertools::Itertools;

use crate::util::read_lines;

const BINGO_SIZE: usize = 5;

#[derive(Debug)]
struct BingoCard {
    // (row, col) -> (number, order)
    nums: HashMap<(usize, usize), (i32, usize)>,
}

fn get_order_and_score(card: BingoCard) -> (usize, i32) {
    // --- get order and value of number that wins
    // find max(row) and max(col) for all rows and cols, then take min of that
    let min_rows = (0..(BINGO_SIZE - 1))
        .into_iter()
        .map(|curr_row| {
            card.nums
                .iter()
                .filter_map(move |((row, _col), (number, order))| {
                    if curr_row == *row {
                        Some((order, number))
                    } else {
                        None
                    }
                })
                .max()
                .unwrap()
        })
        .min()
        .unwrap();

    let min_cols = (0..(BINGO_SIZE - 1))
        .into_iter()
        .map(|curr_col| {
            card.nums
                .iter()
                .filter_map(move |((_row, col), (number, order))| {
                    if curr_col == *col {
                        Some((order, number))
                    } else {
                        None
                    }
                })
                .max()
                .unwrap()
        })
        .min()
        .unwrap();

    let (winner_order, winner_value) = cmp::min(min_rows, min_cols);
    println!(
        "winner order: {}, winner_value: {}",
        winner_order, winner_value
    );

    // --- find unmarked numbers
    let unmarked_numbers: i32 = card
        .nums
        .iter()
        .filter_map(|(_, (number, order))| {
            if order > winner_order {
                Some(number)
            } else {
                None
            }
        })
        .sum();
    println!("unmarked_numbers: {}", unmarked_numbers);

    (*winner_order, unmarked_numbers * winner_value)
}

fn get_card_info<P>(filename: P) -> Vec<BingoCard> where P: AsRef<Path>{
    let lines = read_lines(filename).expect("failed to read input");
    let mut it = lines;
    let number_to_order: HashMap<i32, usize> = it
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .enumerate()
        .map(|(idx, draw)| (draw, idx))
        .collect();

    println!("{:?}", number_to_order);
    it.chunks(BINGO_SIZE + 1)
        .into_iter()
        .map(|mut chunk| {
            let _ = chunk.next().unwrap();
            let nums: HashMap<(usize, usize), (i32, usize)> = chunk
                .into_iter()
                .enumerate()
                .map(|(row_idx, row)| {
                    row.unwrap()
                        .split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .enumerate()
                        .map(|(col_idx, num)| ((row_idx, col_idx), (num, number_to_order[&num])))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect();
            BingoCard { nums }
        })
        .collect()
}

pub fn solution_1<P>(fname: P) -> i32 where P: AsRef<Path> {
    get_card_info(fname)
        .into_iter()
        .map(get_order_and_score)
        .min()
        .unwrap()
        .1
}

pub fn solution_2<P>(fname: P) -> i32 where P: AsRef<Path> {
    get_card_info(fname)
        .into_iter()
        .map(get_order_and_score)
        .max()
        .unwrap()
        .1
}
