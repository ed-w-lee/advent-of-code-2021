use std::{collections::HashMap, path::Path};

use crate::util::read_lines;

fn generated_fish(dp: &mut HashMap<(u32, u32), usize>, timer: u32, num_days: u32) -> usize {
    if num_days == 0 {
        return 1;
    }
    if let Some(cached) = dp.get(&(timer, num_days)) {
        return *cached;
    }
    if timer == 0 {
        let to_ret = generated_fish(dp, 6, num_days - 1) + generated_fish(dp, 8, num_days - 1);
        dp.insert((timer, num_days), to_ret);
        return to_ret;
    }
    generated_fish(dp, timer - 1, num_days - 1)
}

pub fn solution_1<P>(filename: P) -> usize where P: AsRef<Path> {
    const NUM_DAYS: u32 = 80;
    let mut lines = read_lines(filename).expect("failed to read input");
    let line = lines.next().expect("no line").expect("no line 2");
    let mut dp: HashMap<(u32, u32), usize> = HashMap::new();
    line.split_terminator(',')
        .map(|timer| generated_fish(&mut dp, timer.parse::<u32>().unwrap(), NUM_DAYS))
        .sum()
}

pub fn solution_2<P>(filename: P) -> usize where P: AsRef<Path> {
    const NUM_DAYS: u32 = 256;
    let mut lines = read_lines(filename).expect("failed to read input");
    let line = lines.next().expect("no line").expect("no line 2");
    let mut dp: HashMap<(u32, u32), usize> = HashMap::new();
    line.split_terminator(',')
        .map(|timer| generated_fish(&mut dp, timer.parse::<u32>().unwrap(), NUM_DAYS))
        .sum()
}
