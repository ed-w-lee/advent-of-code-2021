use std::{collections::{HashMap, VecDeque}, path::Path};
use itertools::Itertools;

use crate::util::read_lines;

// hinges on a few properties in the input:
// * A is 26 when B is negative, and 1 when B is positive
// * when B is positive, B >= 10
// * C <= 15 (specifically when B is positive), so (input + C) is always < 26
//
// this means that when A = 1, Z' = Z * 26 + (input + C)
// and when A = 26, Z' = Z / 26
//
// As such, Z acts like a stack storing (input + C) values, and we need to make sure when we "pop" from the stack that:
// input_curr + B_curr = input_prev + C_prev
fn necessary_input(vars: &[(i64, i64, i64)], prefer_max: bool) -> String {
    let mut stack: VecDeque<(usize, i64)> = VecDeque::new();
    let mut digits: HashMap<usize, i64> = HashMap::new();

    for (curr_idx, &(a, b, c)) in vars.iter().enumerate() {
        if a == 1 {
            stack.push_back((curr_idx, c));
        } else {
            let (prev_idx, prev_offset) = stack.pop_back().unwrap();
            let net_offset = prev_offset + b;
            let (prev_val, curr_val) = match (prefer_max, net_offset > 0) {
                (true, true) => (9 - net_offset, 9),
                (true, false) => (9, 9 + net_offset),
                (false, true) => (1, 1 + net_offset),
                (false, false) => (1 - net_offset, 1),
            };
            digits.insert(prev_idx, prev_val);
            digits.insert(curr_idx, curr_val);
        }
    }
    
    digits.into_iter().sorted().fold("".to_string(), |acc, (_, val)| {
        acc + &val.to_string()
    })
}

pub fn solution_1<P>(filename: P) -> String where P: AsRef<Path> {
    let lines = read_lines(filename).expect("failed to read input");
    let vars = lines
        .into_iter().chunks(18).into_iter().map(|chunk| {
            let vars = chunk.into_iter().enumerate().filter_map(|(line_idx, l)| {
                match line_idx {
                    4 => Some(("A", l.unwrap().split_whitespace().nth(2).unwrap().parse::<i64>().unwrap())),
                    5 => Some(("B", l.unwrap().split_whitespace().nth(2).unwrap().parse::<i64>().unwrap())),
                    15 => Some(("C", l.unwrap().split_whitespace().nth(2).unwrap().parse::<i64>().unwrap())),
                    _ => None
                }
            }).collect::<HashMap<_, _>>();
            (vars["A"], vars["B"], vars["C"])
        }).collect_vec();


    println!("{:?}", vars);
    necessary_input(&vars, true)
}

pub fn solution_2<P>(filename: P) -> String where P: AsRef<Path> {
    let lines = read_lines(filename).expect("failed to read input");
    let vars = lines
        .into_iter().chunks(18).into_iter().map(|chunk| {
            let vars = chunk.into_iter().enumerate().filter_map(|(line_idx, l)| {
                match line_idx {
                    4 => Some(("A", l.unwrap().split_whitespace().nth(2).unwrap().parse::<i64>().unwrap())),
                    5 => Some(("B", l.unwrap().split_whitespace().nth(2).unwrap().parse::<i64>().unwrap())),
                    15 => Some(("C", l.unwrap().split_whitespace().nth(2).unwrap().parse::<i64>().unwrap())),
                    _ => None
                }
            }).collect::<HashMap<_, _>>();
            (vars["A"], vars["B"], vars["C"])
        }).collect_vec();


    println!("{:?}", vars);
    necessary_input(&vars, false)
}
