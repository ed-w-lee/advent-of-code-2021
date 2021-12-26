use std::{collections::{HashMap, VecDeque}, path::Path, str::FromStr, string::ParseError, ops::Add};
use itertools::Itertools;

use crate::util::read_lines;
use crate::day24_bad::Input::*;
use crate::day24_bad::Var::*;
use crate::day24_bad::Op::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Var {
    W, X, Y, Z
}

#[derive(Debug)]
enum Input {
    Var(Var), Number(i64),
}

#[derive(Debug)]
enum Op {
    Inp(Var),
    Add(Var, Input),
    Mul(Var, Input),
    Div(Var, Input),
    Mod(Var, Input),
    Eql(Var, Input),
}

impl FromStr for Var {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(W),
            "x" => Ok(X),
            "y" => Ok(Y),
            "z" => Ok(Z),
            _ => {
                Err(format!("Failed to parse into var: {}", s))
            },
        }
    }
}

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Var(W)),
            "x" => Ok(Var(X)),
            "y" => Ok(Var(Y)),
            "z" => Ok(Var(Z)),
            _ => {
                match s.parse::<i64>() {
                    Ok(i) => Ok(Number(i)),
                    Err(_) => Err(format!("Failed to parse into integer: {}", s))
                }
            },
        }
    }
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let instruction = match split.next() {
            Some(instr) => instr,
            None => return Err(format!("Unable to split properly")),
        };
        match instruction {
            "inp" => Ok(Inp(split.next().unwrap().parse::<Var>().unwrap())),
            "add" => Ok(Add(split.next().unwrap().parse::<Var>().unwrap(), 
                            split.next().unwrap().parse::<Input>().unwrap())),
            "mul" => Ok(Mul(split.next().unwrap().parse::<Var>().unwrap(), 
                            split.next().unwrap().parse::<Input>().unwrap())),
            "div" => Ok(Div(split.next().unwrap().parse::<Var>().unwrap(), 
                            split.next().unwrap().parse::<Input>().unwrap())),
            "mod" => Ok(Mod(split.next().unwrap().parse::<Var>().unwrap(), 
                            split.next().unwrap().parse::<Input>().unwrap())),
            "eql" => Ok(Eql(split.next().unwrap().parse::<Var>().unwrap(), 
                            split.next().unwrap().parse::<Input>().unwrap())),
            _ => Err(format!("Bad instruction {}", instruction))
        }
    }
}

fn resolve(state: &HashMap<Var, i64>, input: &Input) -> i64 {
    match input {
        Input::Var(v) => state[v],
        Number(n) => *n,
    }
}

fn run_to_next_input(stats: &mut HashMap<&str, u64>, ops: &[Op], state: &mut HashMap<Var, i64>) -> Option<String> {
    for (i, op) in ops.iter().enumerate() {
        match op {
            Inp(v) => {
                // hit the input, recurse on 1-9
                let old_state = state.clone();
                for next_val in (1..10).rev() {
                    state.insert(*v, next_val);
                    match run_to_next_input(stats, &ops[i+1..], state) {
                        Some(str) => {
                            return Some(next_val.to_string() + &str);
                        }
                        None => (),
                    }
                    state.clone_from(&old_state)
                }
            },
            Add(a, b) => {
                let b_val = resolve(state, b);
                let old_val = state[a];
                let new_val = old_val + b_val;
                state.insert(*a, new_val);
            },
            Mul(a, b) => {
                let b_val = resolve(state, b);
                let old_val = state[a];
                let new_val = old_val * b_val;
                state.insert(*a, new_val);
            },
            Div(a, b) => {
                let b_val = resolve(state, b);
                let old_val = state[a];
                let new_val = old_val / b_val;
                state.insert(*a, new_val);
            },
            Mod(a, b) => {
                let b_val = resolve(state, b);
                let old_val = state[a];
                let new_val = old_val % b_val;
                state.insert(*a, new_val);
            },
            Eql(a, b) => {
                let b_val = resolve(state, b);
                let old_val = state[a];
                let new_val = if old_val == b_val { 1 } else { 0 };
                state.insert(*a, new_val);
            },
        }
    }

    let checked = stats.get_mut("checked").unwrap();
    *checked += 1;
    if *checked % 1000 == 0 {
        println!("checked: {}", checked);
    }
        
    if state[&Z] == 0 {
        Some("".to_string())
    } else {
        None
    }
}

pub fn solution_1<P>(filename: P) -> String where P: AsRef<Path> {
    let lines = read_lines(filename).expect("failed to read input");
    let ops = lines
        .into_iter()
        .map(|line| line.unwrap().parse::<Op>().unwrap())
        .collect::<Vec<_>>();
    let mut state = HashMap::from([
        (W, 0),
        (X, 0),
        (Y, 0),
        (Z, 0),
    ]);
    let mut stats = HashMap::from([
        ("checked", 0),
    ]);
    run_to_next_input(&mut stats, &ops, &mut state).unwrap()
}

pub fn solution_2<P>(filename: P) -> u64 where P: AsRef<Path> {
    todo!()
}
