use std::{
    char::from_digit,
    collections::{HashMap, HashSet}, path::Path,
};

use itertools::Itertools;

use crate::util::read_lines;

pub fn solution_1<P>(filename: P) -> usize where P: AsRef<Path> {
    let unique_digits = HashSet::from([2, 3, 4, 7]);
    let lines = read_lines(filename).expect("failed to read input");
    lines
        .into_iter()
        .map(|line| {
            line.unwrap()
                .split_terminator(" | ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|digit| digit.len())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .filter(|unique_lens| unique_digits.contains(unique_lens))
        .count()
}

fn find_digits(digits: Vec<String>) -> HashMap<String, char> {
    // 1, 4, 7, 8 -> by length
    // we know a, (c/f), (b/d), (e/g)
    // 0 -> by having a, (c/f), (e/g)
    // we know a, b, d, (c/f), (e/g)
    // 6 -> by having a, (b/d), (e/g)
    // we know a, b, d, c, f, (e/g)
    // 9 -> by having a, (b/d), (c/f)
    // we know a, b, d, c, f, e, g
    let mut digit_letters: HashMap<u8, HashSet<char>> = HashMap::new();
    for digit in digits.iter() {
        match digit.len() {
            2 => {
                digit_letters.insert(1, digit.chars().into_iter().collect());
            }
            3 => {
                digit_letters.insert(7, digit.chars().into_iter().collect());
            }
            4 => {
                digit_letters.insert(4, digit.chars().into_iter().collect());
            }
            7 => {
                digit_letters.insert(8, digit.chars().into_iter().collect());
            }
            _ => (),
        }
    }
    let letter_a = digit_letters[&7]
        .difference(&digit_letters[&1])
        .next()
        .unwrap();
    let c_or_f = digit_letters[&1].clone();
    let b_or_d: HashSet<char> = digit_letters[&4]
        .difference(&digit_letters[&1]).copied()
        .collect();
    let mut e_or_g: HashSet<char> = digit_letters[&8]
        .difference(&digit_letters[&4]).copied()
        .collect();
    e_or_g.remove(letter_a);
    let mut letter_b: char = 'x';
    let mut letter_e: char = 'x';
    for digit in digits.iter() {
        match digit.len() {
            2 | 3 | 4 | 7 => (),
            6 => {
                let chars: HashSet<char> = digit.chars().into_iter().collect();
                let missing = digit_letters[&8].difference(&chars).next().unwrap();
                if e_or_g.contains(missing) {
                    letter_e = *missing;
                    digit_letters.insert(9, chars);
                } else if b_or_d.contains(missing) {
                    letter_b = *b_or_d.iter().find(|c| *c != missing).unwrap();
                    digit_letters.insert(0, chars);
                } else if c_or_f.contains(missing) {
                    digit_letters.insert(6, chars);
                } else {
                    unreachable!()
                }
            }
            _ => (),
        }
    }
    for digit in digits.iter() {
        match digit.len() {
            2 | 3 | 4 | 7 | 6 => (),
            5 => {
                let chars: HashSet<char> = digit.chars().into_iter().collect();
                let has_b = chars.contains(&letter_b);
                let has_e = chars.contains(&letter_e);
                match (has_b, has_e) {
                    (false, false) => {
                        digit_letters.insert(3, chars);
                    }
                    (false, true) => {
                        digit_letters.insert(2, chars);
                    }
                    (true, false) => {
                        digit_letters.insert(5, chars);
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    digit_letters
        .into_iter()
        .map(|(digit, letters)| {
            (
                letters.into_iter().sorted().collect(),
                from_digit(digit as u32, 10).unwrap(),
            )
        })
        .collect()
}

pub fn solution_2<P>(filename: P) -> u32 where P: AsRef<Path> {
    let lines = read_lines(filename).expect("failed to read input");
    let digit_outputs: Vec<(Vec<String>, Vec<String>)> = lines
        .into_iter()
        .map(|line_res| {
            let line = line_res.unwrap();
            let mut digit_output = line.split_terminator(" | ");
            (
                digit_output
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
                digit_output
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
            )
        })
        .collect();
    digit_outputs
        .into_iter()
        .map(|(digits, outputs)| {
            let wires_to_digits = find_digits(digits);
            outputs
                .into_iter()
                .map(move |output| {
                    let sorted_chars: String = output.chars().sorted().collect();
                    *wires_to_digits
                        .get(&sorted_chars)
                        .expect("no sorted char in wires_to_digits")
                })
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}
