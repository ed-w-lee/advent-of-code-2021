use std::{
    cmp::min,
    collections::{HashMap, VecDeque},
    path::Path,
};

use itertools::Itertools;

use crate::util::read_lines;

fn num_possible_x(xbound: (u64, u64), step_bound: (u64, u64)) -> usize {
    // you might be able to cache the (step, bound) -> count, but whatever
    println!("steps: {:?}", step_bound);
    let (x_low, x_high) = xbound;
    let (raw_step_low, raw_step_high) = step_bound;
    // x + max((x - 1), 0) + max((x - 2), 0) + ...
    // x + (x - 1) + (x - 2)
    // steps * x - (step * (step - 1) / 2)
    let mut to_ret = 0;
    for x_vel in 1..(x_high + 1) {
        let step_low = min(raw_step_low, x_vel);
        let step_high = min(raw_step_high, x_vel);
        for step in step_low..(step_high + 1) {
            let x_pos = step * x_vel - (step * (step - 1)) / 2;
            if x_low <= x_pos && x_pos <= x_high {
                println!("x: {:?}", x_vel);
                to_ret += 1;
                break;
            }
        }
    }
    to_ret
}

fn num_steps(y_vel: i64, ybound: (i64, i64)) -> Option<(u64, u64)> {
    // y_low <= y_vel * step - (step * (step - 1) / 2) <= y_high
    // 2 * y_vel * step - step * step + step - 2 * y_bound
    // (rev) step * step - (2 * y_vel + 1) * step + 2 * y_bound
    // ((2 * y_vel + 1) + sqrt( (2 * y_vel + 1)^2 - 4 * 2 * y_bound )) / 2
    let (y_low, y_high) = ybound;
    let b = (2 * y_vel + 1) as f64;
    let step_high = ((b + (b.powi(2) - 8.0 * (y_low as f64)).sqrt()) / 2.0).floor() as u64;
    let step_low = ((b + (b.powi(2) - 8.0 * (y_high as f64)).sqrt()) / 2.0).ceil() as u64;
    if step_low <= step_high {
        Some((step_low, step_high))
    } else {
        // no steps satisfy the bounds
        None
    }
}

pub fn solution_1(_: (u64, u64), ybound: (i64, i64)) -> i64 {
    let (y_low, _) = ybound;
    y_low * (y_low + 1) / 2
}

pub fn solution_2(xbound: (u64, u64), ybound: (i64, i64)) -> usize {
    // const X_LOW: u64 = 22; // (22 * 23) / 2 > 244
    let (y_low, y_high) = ybound;
    assert!(y_low < 0 && y_high < 0);
    let mut total = 0;
    for y_vel in y_low..-y_low {
        println!("y_vel: {}", y_vel);
        match num_steps(y_vel, ybound) {
            Some(step_bound) => {
                total += num_possible_x(xbound, step_bound);
            }
            None => (),
        }
        println!("total: {:?}", total);
    }
    total
}
