use std::path::Path;

use crate::util::read_lines;

#[derive(Add, Sum)]
struct Offset(i32, i32);

pub fn solution_1<P>(fname: P) -> i32
where
    P: AsRef<Path>,
{
    let lines = read_lines(fname).expect("failed to read input");
    let offset = lines
        .into_iter()
        .map(|line| {
            line.expect("couldn't find line")
                .split_whitespace()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|split_lines| {
            (
                split_lines[0].clone(),
                split_lines[1]
                    .parse::<i32>()
                    .expect("couldn't parse magnitude"),
            )
        })
        .map(|(direction, magnitude)| match direction.as_str() {
            "forward" => Offset(magnitude, 0),
            "down" => Offset(0, magnitude),
            "up" => Offset(0, -magnitude),
            _ => unimplemented!(),
        })
        .sum::<Offset>();
    offset.0 * offset.1
}

pub fn solution_2<P>(fname: P) -> i32
where
    P: AsRef<Path>,
{
    let lines = read_lines(fname).expect("failed to read input");
    let offset = lines
        .into_iter()
        .map(|line| {
            line.expect("couldn't find line")
                .split_whitespace()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|split_lines| {
            (
                split_lines[0].clone(),
                split_lines[1]
                    .parse::<i32>()
                    .expect("couldn't parse magnitude"),
            )
        })
        .fold(
            (0, Offset(0, 0)),
            |state, (direction, magnitude)| match direction.as_str() {
                "down" => (state.0 + magnitude, state.1),
                "up" => (state.0 - magnitude, state.1),
                "forward" => (
                    state.0,
                    Offset((state.1).0 + magnitude, (state.1).1 + (state.0 * magnitude)),
                ),
                _ => unimplemented!(),
            },
        )
        .1;
    offset.0 * offset.1
}
