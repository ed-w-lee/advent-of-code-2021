use std::{collections::HashSet, path::Path};

use itertools::Itertools;

use crate::util::read_lines;

// axis[0] = x if true else y
fn fold_paper(paper: &HashSet<(usize, usize)>, axis: (bool, usize)) -> HashSet<(usize, usize)> {
    paper
        .iter()
        .filter_map(|(x, y)| match axis {
            (true, line) => {
                // x axis fold
                match *x {
                    a if a == line => None,
                    a if a < line => Some((*x, *y)),
                    _ => Some((2 * line - *x, *y)),
                }
            }
            (false, line) => {
                // y axis fold
                match *y {
                    a if a == line => None,
                    a if a < line => Some((*x, *y)),
                    _ => Some((*x, 2 * line - *y)),
                }
            }
        })
        .collect()
}

pub fn solution_1<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let mut lines = read_lines(filename).expect("failed to read input");
    let mut paper: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.trim().is_empty() {
            break;
        }
        let mut coords = line.split(',');
        paper.insert((
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
        ));
    }

    let line = lines.next().unwrap().unwrap();
    let fold_along = &line[11..];
    let mut axis = fold_along.split('=');
    let orientation: bool = match axis.next().unwrap().parse().unwrap() {
        'x' => true,
        'y' => false,
        _ => unreachable!(),
    };
    let line: usize = axis.next().unwrap().parse().unwrap();
    let papers = fold_paper(&paper, (orientation, line));
    papers.len()
}

fn render_coords(coords: HashSet<(usize, usize)>) -> String {
    let max_x = *(coords.iter().map(|(x, _)| x).max().unwrap()) as usize + 1;
    let max_y = *(coords.iter().map(|(_, y)| y).max().unwrap()) as usize + 1;
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; max_x]; max_y];
    coords.into_iter().for_each(|(x, y)| (canvas[y])[x] = 'x');
    canvas
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .join("\n")
}

pub fn solution_2<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    let mut lines = read_lines(filename).expect("failed to read input");
    let mut paper: HashSet<(usize, usize)> = HashSet::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.trim().is_empty() {
            break;
        }
        let mut coords = line.split(',');
        paper.insert((
            coords.next().unwrap().parse().unwrap(),
            coords.next().unwrap().parse().unwrap(),
        ));
    }

    lines.for_each(|line_res| {
        let line = line_res.unwrap();
        let fold_along = &line[11..];
        let mut axis = fold_along.split('=');
        let orientation: bool = match axis.next().unwrap().parse().unwrap() {
            'x' => true,
            'y' => false,
            _ => unreachable!(),
        };
        let pos: usize = axis.next().unwrap().parse().unwrap();
        paper = fold_paper(&paper, (orientation, pos));
    });
    render_coords(paper)
}
