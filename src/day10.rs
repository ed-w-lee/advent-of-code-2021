use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::util::read_lines;

fn parse_line(closing_map: &HashMap<char, char>, line: String) -> Result<VecDeque<char>, char> {
    line.chars()
        .try_fold(VecDeque::<char>::new(), |mut stack, c| match c {
            '(' | '[' | '{' | '<' => {
                stack.push_back(c);
                Ok(stack)
            }
            ')' | ']' | '}' | '>' => {
                if let Some(stack_c) = stack.pop_back() {
                    if c == *closing_map.get(&stack_c).unwrap() {
                        Ok(stack)
                    } else {
                        Err(c)
                    }
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        })
}

pub fn solution_1() -> u32 {
    let point_map: HashMap<char, u32> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let closing_map: HashMap<char, char> =
        HashMap::from([('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]);

    let lines = read_lines("input/day10_input.txt").expect("failed to read input");
    lines
        .into_iter()
        .map(|line| parse_line(&closing_map, line.unwrap()))
        .map(|res| {
            if let Err(c) = res {
                *point_map.get(&c).unwrap()
            } else {
                0
            }
        })
        .sum()
}

pub fn solution_2() -> u64 {
    let point_map: HashMap<char, u64> = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let closing_map: HashMap<char, char> =
        HashMap::from([('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]);

    let lines = read_lines("input/day10_input.txt").expect("failed to read input");
    let scores: Vec<u64> = lines
        .into_iter()
        .filter_map(|line| parse_line(&closing_map, line.unwrap()).ok())
        .map(|res| {
            res.into_iter().rev().fold(0u64, |acc, c| {
                acc * 5 + point_map.get(&closing_map.get(&c).unwrap()).unwrap()
            })
        })
        .sorted()
        .collect();
    scores[scores.len() / 2]
}
