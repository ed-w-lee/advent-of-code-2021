use std::{collections::HashMap, path::Path};

use itertools::Itertools;

use crate::util::read_lines;

fn get_counts(
    insertion_rules: &HashMap<(char, char), char>,
    template: &str,
    num_steps: usize,
) -> HashMap<char, usize> {
    let mut template_chars = template.chars();
    let start_char = template_chars.next().unwrap();
    let end_char = template_chars.last().unwrap();

    let mut counts: HashMap<(char, char), usize> = template
        .chars()
        .tuple_windows()
        .map(|pair: (char, char)| (pair, 1))
        .collect();

    for _ in 0..num_steps {
        let mut new_counts: HashMap<(char, char), usize> = HashMap::new();
        counts
            .into_iter()
            .for_each(|((a, b), count)| match insertion_rules.get(&(a, b)) {
                Some(to_insert) => {
                    let left = (a, *to_insert);
                    let right = (*to_insert, b);
                    *(new_counts.entry(left).or_insert(0)) += count;
                    *(new_counts.entry(right).or_insert(0)) += count;
                }
                None => *(new_counts.entry((a, b)).or_insert(0)) += count,
            });
        counts = new_counts;
    }
    let mut char_counts = counts
        .into_iter()
        .map(|((a, b), count)| vec![(a, count), (b, count)])
        .flatten()
        .fold(HashMap::<char, usize>::new(), |mut acc, (c, count)| {
            *(acc.entry(c).or_insert(0)) += count;
            acc
        });
    char_counts
        .entry(start_char)
        .and_modify(|count| *count += 1);
    char_counts.entry(end_char).and_modify(|count| *count += 1);
    char_counts
        .into_iter()
        .map(|(a, count)| (a, count / 2))
        .collect()
}

pub fn solution_1<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let mut lines = read_lines(filename).expect("failed to read input");
    let template = lines.next().unwrap().unwrap();
    let _ = lines.next();
    let insertion_rules: HashMap<(char, char), char> = lines
        .into_iter()
        .map(|line| {
            let line_str = line.unwrap();
            let mut rule_pair = line_str.split_terminator(" -> ");
            let pair_str = rule_pair.next().unwrap();
            let insert = rule_pair.next().unwrap().chars().next().unwrap();
            let pair: (char, char) = pair_str.chars().collect_tuple().unwrap();
            (pair, insert)
        })
        .collect();

    let mut c_iter = get_counts(&insertion_rules, &template, 10)
        .into_iter()
        .map(|(_, count)| count)
        .sorted();

    let min_count = c_iter.next().unwrap();
    let max_count = c_iter.last().unwrap();

    max_count - min_count
}

pub fn solution_2<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let mut lines = read_lines(filename).expect("failed to read input");
    let template = lines.next().unwrap().unwrap();
    let _ = lines.next();
    let insertion_rules: HashMap<(char, char), char> = lines
        .into_iter()
        .map(|line| {
            let line_str = line.unwrap();
            let mut rule_pair = line_str.split_terminator(" -> ");
            let pair_str = rule_pair.next().unwrap();
            let insert = rule_pair.next().unwrap().chars().next().unwrap();
            let pair: (char, char) = pair_str.chars().collect_tuple().unwrap();
            (pair, insert)
        })
        .collect();

    let mut c_iter = get_counts(&insertion_rules, &template, 40)
        .into_iter()
        .map(|(_, count)| count)
        .sorted();

    let min_count = c_iter.next().unwrap();
    let max_count = c_iter.last().unwrap();

    max_count - min_count
}
