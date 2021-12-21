use std::{cmp::max, collections::HashMap, iter::repeat_with};

use itertools::Itertools;

#[derive(Add, Sum, Clone, Copy)]
struct Scores(u64, u64);

fn simulate_to_win(
    p1_pos: u64,
    p2_pos: u64,
    iterator: &mut dyn Iterator<Item = u64>,
) -> (u64, u64) {
    let mut player_1_pos = p1_pos;
    let mut player_1_score = 0;
    let mut player_2_pos = p2_pos;
    let mut player_2_score = 0;

    for turns in 1.. {
        let next_3_rolls: Vec<_> = iterator.take(3).collect();
        let next_roll: u64 = next_3_rolls.iter().sum();
        match turns % 2 {
            1 => {
                player_1_pos = (player_1_pos + next_roll - 1) % 10 + 1;
                player_1_score += player_1_pos;
                if player_1_score >= 1000 {
                    return (player_2_score, turns * 3);
                }
            }
            0 => {
                player_2_pos = (player_2_pos + next_roll - 1) % 10 + 1;
                player_2_score += player_2_pos;
                if player_2_score >= 1000 {
                    return (player_1_score, turns * 3);
                }
            }
            _ => unreachable!(),
        };
    }
    unreachable!()
}

pub fn solution_1(p1: u64, p2: u64) -> u64 {
    let mut curr = 1;
    let mut deterministic_dice = repeat_with(|| {
        let tmp = curr;
        curr = curr % 100 + 1;
        println!("curr: {}", curr);
        tmp
    });
    let (losing_score, turns) = simulate_to_win(p1, p2, &mut deterministic_dice);
    println!("{} {}", losing_score, turns);
    losing_score * turns
}

fn num_wins(
    dp: &mut HashMap<(bool, u64, u64, u64, u64), Scores>,
    possibilities: &HashMap<u64, u64>,
    is_p1_turn: bool,
    p1_pos: u64,
    p1_score: u64,
    p2_pos: u64,
    p2_score: u64,
) -> Scores {
    if !is_p1_turn && p1_score >= 21 {
        return Scores(1, 0);
    }
    if is_p1_turn && p2_score >= 21 {
        return Scores(0, 1);
    }
    if dp.contains_key(&(is_p1_turn, p1_pos, p1_score, p2_pos, p2_score)) {
        return *dp
            .get(&(is_p1_turn, p1_pos, p1_score, p2_pos, p2_score))
            .unwrap();
    }
    let res: Scores = possibilities
        .iter()
        .map(|(possibility, count)| {
            let Scores(p1_wins, p2_wins) = {
                if is_p1_turn {
                    let next_p1_pos = (p1_pos + possibility - 1) % 10 + 1;
                    num_wins(
                        dp,
                        possibilities,
                        !is_p1_turn,
                        next_p1_pos,
                        p1_score + next_p1_pos,
                        p2_pos,
                        p2_score,
                    )
                } else {
                    let next_p2_pos = (p2_pos + possibility - 1) % 10 + 1;
                    num_wins(
                        dp,
                        possibilities,
                        !is_p1_turn,
                        p1_pos,
                        p1_score,
                        next_p2_pos,
                        p2_score + next_p2_pos,
                    )
                }
            };
            Scores(p1_wins * count, p2_wins * count)
        })
        .sum();

    dp.insert((is_p1_turn, p1_pos, p1_score, p2_pos, p2_score), res);
    res
}

pub fn solution_2(p1: u64, p2: u64) -> u64 {
    let mut dp: HashMap<(bool, u64, u64, u64, u64), Scores> = HashMap::new();
    let possibilities = (1..4)
        .cartesian_product(1..4)
        .cartesian_product(1..4)
        .map(|((a, b), c)| a + b + c)
        .fold(HashMap::<u64, u64>::new(), |mut acc, sum| {
            *(acc.entry(sum).or_insert(0)) += 1;
            acc
        });
    let Scores(p1_wins, p2_wins) = num_wins(&mut dp, &possibilities, true, p1, 0, p2, 0);
    max(p1_wins, p2_wins)
}
