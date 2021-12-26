use std::{
    char::from_digit,
    cmp::{max, Ordering},
    collections::{BinaryHeap, HashMap},
    path::Path,
};

use itertools::Itertools;

use crate::util::read_lines;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    cost: u64,
    position: (i32, i32),
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| {
            let my_pos = self.position.0 + self.position.1;
            let other_pos = other.position.0 + other.position.1;
            my_pos.cmp(&other_pos)
        })
    }
}

fn render_map(costs: &HashMap<(i32, i32), u64>) -> String {
    let max_x = *(costs.iter().map(|((x, _), _)| x).max().unwrap()) as usize + 1;
    let max_y = *(costs.iter().map(|((_, y), _)| y).max().unwrap()) as usize + 1;
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; max_x]; max_y];
    costs.iter().for_each(|((x, y), cost)| {
        (canvas[*x as usize])[*y as usize] = from_digit(*cost as u32, 10).unwrap()
    });
    canvas
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .join("\n")
}

fn shortest_path(costs: &HashMap<(i32, i32), u64>, start: &(i32, i32), goal: &(i32, i32)) -> u64 {
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<_, _> = costs.keys().map(|(r, c)| ((*r, *c), u64::MAX)).collect();
    *(dist.get_mut(start).unwrap()) = 0;

    heap.push(State {
        cost: 0,
        position: *start,
    });

    let surround: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some(State { cost, position }) = heap.pop() {
        if position == *goal {
            return cost;
        }
        if cost > *dist.get(&position).unwrap() {
            continue;
        }
        let (r, c) = position;
        surround
            .iter()
            .filter_map(|(offr, offc)| {
                let next_pos = (r + offr, c + offc);
                costs.get(&next_pos).map(|next_cost| State {
                        cost: cost + *next_cost,
                        position: next_pos,
                    })
            })
            .for_each(|next_state| {
                if next_state.cost < *dist.get(&next_state.position).unwrap() {
                    heap.push(next_state);
                    *dist.get_mut(&next_state.position).unwrap() = next_state.cost;
                }
            });
    }
    unreachable!()
}

pub fn solution_1<P>(filename: P) -> u64
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename).expect("failed to read input");
    let costs: HashMap<(i32, i32), u64> = lines
        .into_iter()
        .enumerate()
        .map(|(row_idx, line)| {
            line.unwrap()
                .char_indices()
                .map(|(col_idx, c)| {
                    (
                        (row_idx as i32, col_idx as i32),
                        c.to_digit(10).unwrap() as u64,
                    )
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    let goal = costs.keys().max().unwrap();

    shortest_path(&costs, &(0, 0), goal)
}

pub fn solution_2<P>(filename: P) -> u64
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename).expect("failed to read input");
    let mut num_rows: i32 = 0;
    let mut num_cols: i32 = 0;
    let costs: HashMap<(i32, i32), u64> = lines
        .into_iter()
        .enumerate()
        .map(|(row_idx, line)| {
            num_rows = max(num_rows, row_idx as i32);
            line.unwrap()
                .char_indices()
                .map(|(col_idx, c)| {
                    num_cols = max(num_cols, col_idx as i32);
                    (
                        (row_idx as i32, col_idx as i32),
                        c.to_digit(10).unwrap() as u64,
                    )
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();
    // println!("{}", render_map(&costs));
    num_rows += 1;
    num_cols += 1;

    let others = costs
        .into_iter()
        .map(|((orig_r, orig_c), orig_cost)| {
            (0..5)
                .into_iter()
                .map(|cave_r| {
                    (0..5)
                        .into_iter()
                        .map(|cave_c| {
                            (
                                (cave_r * num_rows + orig_r, cave_c * num_cols + orig_c),
                                (orig_cost + (cave_r as u64) + (cave_c as u64) - 1) % 9 + 1,
                            )
                        })
                        .collect::<Vec<((i32, i32), u64)>>()
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<HashMap<(i32, i32), u64>>();

    // println!("{}", render_map(&others));
    let goal = others.keys().max().unwrap();

    shortest_path(&others, &(0, 0), goal)
}

// pub fn solution_2<P>(filename: P) -> usize
// where
//     P: AsRef<Path>,
// {
//     let mut lines = read_lines(filename).expect("failed to read input");
//     unimplemented!()
// }
