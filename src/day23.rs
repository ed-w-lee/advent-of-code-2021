use std::{collections::{HashMap, VecDeque}, hash::Hash};

use itertools::Itertools;

use crate::day23::Amphipod::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl From<usize> for Amphipod {
    fn from(variant: usize) -> Self {
        match variant {
            0 => A,
            1 => B,
            2 => C,
            3 => D,
            _ => panic!("blah"),
        }
    }
}

struct Board {
    hallway_len: u64,
    open_spots: [u64; 4],
    sideroom_depth: [u64; 4],
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    hallway_pos: HashMap<u64, Amphipod>,
    sideroom_pop: [VecDeque<Amphipod>; 4],
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let pos_vec = self.hallway_pos.clone().into_iter().sorted().collect::<Vec<_>>();
        pos_vec.hash(state);
        self.sideroom_pop.hash(state);
    }
}

fn is_success(state: &State) -> bool {
    if !state.hallway_pos.is_empty() {
        false
    } else if !state.sideroom_pop[0].iter().all(|a| A == *a) {
        false
    } else if !state.sideroom_pop[1].iter().all(|b| B == *b) {
        false
    } else if !state.sideroom_pop[2].iter().all(|c| C == *c) {
        false
    } else { !(!state.sideroom_pop[3].iter().all(|d| D == *d)) }
}

fn get_min_cost(dp: &mut HashMap<State, (Option<u64>, Option<State>)>, move_cost: &HashMap<Amphipod, u64>, board: &Board, state: &mut State) -> Option<u64> {
    // println!("{:?}", state);
    if is_success(state) {
        return Some(0);
    }
    match dp.get(state) {
        Some(cost) => {
            return (*cost).0;
        },
        None => ()
    };
    let mut min_cost = u64::MAX;
    let mut next_state = None;
    let available_hallway: Vec<u64> = (0..board.hallway_len).filter(|pos| {
        !(board.open_spots.contains(pos) || state.hallway_pos.contains_key(pos))
    }).collect();
    if !available_hallway.is_empty() {
        // moves that "pop" out from sideroom
        (0..4).for_each(|idx| {
            if !state.sideroom_pop[idx].is_empty() && !state.sideroom_pop[idx].iter().all(|a| *a as usize == idx) {
                let curr_len = state.sideroom_pop[idx].len();
                let to_hallway_units = board.sideroom_depth[idx] - curr_len as u64 + 1;
                let popped = state.sideroom_pop[idx].pop_back().unwrap();
                let pos = board.open_spots[idx];
                for &dest_spot in available_hallway.iter() {
                    let (hallway_offset, is_visitable) = if dest_spot < pos {
                        ( ((pos as i64) - ((dest_spot as i64))) as u64 , state.hallway_pos.keys().all(|&blocking| blocking <= dest_spot || blocking >= pos))
                    } else {
                        ( ((dest_spot as i64) - ((pos as i64))) as u64 , state.hallway_pos.keys().all(|&blocking| blocking >= dest_spot || blocking <= pos))
                    };
                    if is_visitable {
                        state.hallway_pos.insert(dest_spot, popped);
                        let this_move_cost = (hallway_offset + to_hallway_units) * move_cost[&popped];
                        let future_moves = get_min_cost(dp, move_cost, board, state);
                        if future_moves.is_some() {
                            let full_cost = this_move_cost + future_moves.unwrap();
                            if min_cost > full_cost {
                                min_cost = full_cost;
                                next_state = Some(state.clone());
                            }
                        }
                        state.hallway_pos.remove(&dest_spot);
                    }
                }
                state.sideroom_pop[idx].push_back(popped);
            }
        });
    }
    state.hallway_pos.clone().into_iter().sorted().for_each(|(pos, amph)| {
        let idx = amph as usize;
        if state.sideroom_pop[idx].iter().all(|a| *a as usize == idx) && state.sideroom_pop[idx].len() < board.sideroom_depth[idx] as usize {
            // we can move into the correct spot and there is space
            let dest_spot = board.open_spots[idx];
            let (hallway_offset, is_visitable) = if dest_spot < pos {
                ( ((pos as i64) - ((dest_spot as i64))).abs() as u64 , state.hallway_pos.keys().all(|&blocking| blocking <= dest_spot || blocking >= pos))
            } else {
                ( ((dest_spot as i64) - ((pos as i64))).abs() as u64 , state.hallway_pos.keys().all(|&blocking| blocking >= dest_spot || blocking <= pos))
            };
            if is_visitable {
                state.hallway_pos.remove(&pos);
                state.sideroom_pop[idx].push_back(amph);
                let from_hallway_units = board.sideroom_depth[idx] - state.sideroom_pop[idx].len() as u64 + 1;
                let this_move_cost = (hallway_offset + from_hallway_units) * move_cost[&amph];
                let future_moves = get_min_cost(dp, move_cost, board, state);
                if future_moves.is_some() {
                    let full_cost = this_move_cost + future_moves.unwrap();
                    if min_cost > full_cost {
                        min_cost = full_cost;
                        next_state = Some(state.clone());
                    }
                }
                state.sideroom_pop[idx].pop_back();
                state.hallway_pos.insert(pos, amph);
            }
        }
    });
    let to_ret = if min_cost == u64::MAX {
        None
    } else {
        Some(min_cost)
    };
    dp.insert(state.clone(), (to_ret, next_state));
    if dp.len() % 1000 == 0 {
        println!("states visited: {:?}", dp.len());
    }
    to_ret
}

pub fn solution_1() -> u64 {
    let move_cost = HashMap::from([
        (A, 1),
        (B, 10),
        (C, 100),
        (D, 1000),
    ]);

    // no generic solution because I don't know what part 2 is gonna be like
    let board = Board {
        hallway_len: 11,
        open_spots: [2, 4, 6, 8],
        sideroom_depth: [2, 2, 2, 2],
    };

    let mut state = State {
        hallway_pos: HashMap::new(),
        sideroom_pop: [
            VecDeque::from([B, D]),
            VecDeque::from([C, C]),
            VecDeque::from([D, A]),
            VecDeque::from([A, B]),
        ],
    };

    let mut dp = HashMap::new();
    let to_ret = get_min_cost(&mut dp, &move_cost, &board, &mut state);
    let mut state_iter = state;
    loop {
        match dp.remove(&state_iter) {
            Some((cost, new_state)) => {
                println!("{} {:?}", cost.unwrap(), state_iter);
                let blah = new_state;
                state_iter = blah.unwrap();
            },
            None => break,
        }
    }
    to_ret.unwrap()
}

pub fn solution_2() -> u64 {
    let move_cost = HashMap::from([
        (A, 1),
        (B, 10),
        (C, 100),
        (D, 1000),
    ]);

    let board = Board {
        hallway_len: 11,
        open_spots: [2, 4, 6, 8],
        sideroom_depth: [4, 4, 4, 4],
    };

    let mut state = State {
        hallway_pos: HashMap::new(),
        sideroom_pop: [
            // VecDeque::from([A, D, D, B]),
            // VecDeque::from([D, B, C, C]),
            // VecDeque::from([C, A, B, B]),
            // VecDeque::from([A, C, A, D]),

            VecDeque::from([B, D, D, D]),
            VecDeque::from([C, B, C, C]),
            VecDeque::from([D, A, B, A]),
            VecDeque::from([A, C, A, B]),
        ],
    };

    let mut dp = HashMap::new();
    let to_ret = get_min_cost(&mut dp, &move_cost, &board, &mut state);
    println!("{:?}", to_ret);
    let mut state_iter = state;
    loop {
        match dp.remove(&state_iter) {
            Some((cost, new_state)) => {
                println!("{} {:?}", cost.unwrap(), state_iter);
                let blah = new_state;
                state_iter = blah.unwrap();
            },
            None => break,
        }
    }
    to_ret.unwrap()
}
