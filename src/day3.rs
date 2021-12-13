use std::{vec, path::Path};

use crate::util::read_lines;

pub fn solution_1<P>(fname: P) -> i32 where P: AsRef<Path> {
    let lines = read_lines(fname).expect("failed to read input");
    let nums: Vec<String> = lines
        .into_iter()
        .map(|line| line.expect("couldn't find line"))
        .collect();
    let num_nums = nums.len();
    let mut one_counts = vec![0; nums[0].len()];
    nums.into_iter()
        .map(|bin| bin.chars().collect::<Vec<_>>())
        .for_each(|f| {
            f.into_iter().enumerate().for_each(|(idx, bit)| match bit {
                '0' => (),
                '1' => {
                    one_counts[idx] += 1;
                }
                _ => panic!("bad bit: {:?}", bit),
            })
        });

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for (idx, count) in one_counts.into_iter().rev().enumerate() {
        if count == (num_nums / 2) {
            panic!("no tiebreaker logic")
        } else if count > (num_nums / 2) {
            gamma_rate += 1 << idx;
        } else {
            epsilon_rate += 1 << idx;
        }
    }
    gamma_rate * epsilon_rate
}

enum Rating {
    OXYGEN,
    CO2,
}

fn vec_to_usize(bits: &Vec<u8>) -> usize {
    let mut to_ret: usize = 0;
    for (idx, bit) in bits.iter().rev().enumerate() {
        if *bit == 1 {
            to_ret += 1 << idx;
        }
    }
    to_ret
}

fn filter_step(nums: Vec<Vec<u8>>, idx: usize, rating: Rating) -> usize {
    let num_nums = nums.len();
    if num_nums == 1 {
        return vec_to_usize(&nums[0]);
    }
    let num_ones: usize = nums.iter().map(|v| v[idx] as usize).sum();
    let bit_to_filter: u8 = match num_ones {
        x if 2 * x == num_nums => match rating {
            Rating::OXYGEN => 1,
            Rating::CO2 => 0,
        },
        x if 2 * x < num_nums => match rating {
            Rating::OXYGEN => 0,
            Rating::CO2 => 1,
        },
        x if 2 * x > num_nums => match rating {
            Rating::OXYGEN => 1,
            Rating::CO2 => 0,
        },
        _ => panic!("ranges should be all complete"),
    };
    filter_step(
        nums.into_iter()
            .filter(|num| num[idx] == bit_to_filter)
            .collect(),
        idx + 1,
        rating,
    )
}

pub fn solution_2<P>(fname: P) -> usize where P: AsRef<Path> {
    let lines = read_lines(fname).expect("failed to read input");
    let nums: Vec<Vec<u8>> = lines
        .into_iter()
        .map(|line| {
            line.expect("couldn't find line")
                .chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("bad bit: {:?}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let oxygen = filter_step(nums.clone(), 0, Rating::OXYGEN);
    let co2 = filter_step(nums, 0, Rating::CO2);
    oxygen * co2
}
