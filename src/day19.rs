use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::Path,
};

use itertools::Itertools;

use crate::util::read_lines;

#[derive(Debug, Clone)]
struct Report {
    coords: Vec<(i32, i32, i32)>,
    distance_set: HashSet<i32>,
    distance_map: HashMap<i32, (usize, usize)>,
    node_distances: HashMap<usize, HashMap<i32, usize>>,
    scanner_loc: Option<(i32, i32, i32)>,
}

impl Report {
    fn new(coords: &[(i32, i32, i32)]) -> Self {
        let (distance_set, distance_map, node_distances) = distances_for(&coords);
        Self {
            coords: coords.into_iter().map(|c| c.clone()).collect(),
            distance_set,
            distance_map,
            node_distances,
            scanner_loc: None,
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        self.distance_set.intersection(&other.distance_set).count() >= 66
    }

    fn normalize(&self, other: &mut Self) -> bool {
        if let Some(_) = other.scanner_loc {
            return false;
        }
        let mut intersection = self.distance_set.intersection(&other.distance_set);
        let n_distances = intersection.clone().count();
        if n_distances < 66 {
            return false;
        }
        // refine the node mapping
        let distance1 = intersection.next().unwrap();
        // 11 shared distances between nodes mean nodes are probably equal
        let (self_coord1, self_coord2) = self.distance_map.get(distance1).unwrap();
        let (cand_coord1, cand_coord2) = other.distance_map.get(distance1).unwrap();
        let self_distances: HashSet<_> = self
            .node_distances
            .get(&self_coord1)
            .unwrap()
            .keys()
            .collect();
        let cand_distances: HashSet<_> = other
            .node_distances
            .get(&cand_coord1)
            .unwrap()
            .keys()
            .collect();
        let (s1, s2, o1, o2) = {
            if self_distances.intersection(&cand_distances).count() >= 11 {
                (self_coord1, self_coord2, cand_coord1, cand_coord2)
            } else {
                (self_coord2, self_coord1, cand_coord1, cand_coord2)
            }
        };
        let (dist3, s3) = {
            self.node_distances
                .get(s1)
                .unwrap()
                .iter()
                .filter(|(dist, idx)| s2 != *idx && other.distance_set.contains(*dist))
                .next()
                .unwrap()
        };
        let o1_distances = other.node_distances.get(o1).unwrap();
        let o3 = o1_distances.get(dist3).unwrap();

        let from = [other.coords[*o1], other.coords[*o2], other.coords[*o3]];
        let to = [self.coords[*s1], self.coords[*s2], self.coords[*s3]];
        other.transform(from, to);
        true
    }

    fn transform(&mut self, from: [(i32, i32, i32); 3], to: [(i32, i32, i32); 3]) {
        for rotate_idx in 0..24 {
            let rotated = [
                rotate(&from[0], rotate_idx),
                rotate(&from[1], rotate_idx),
                rotate(&from[2], rotate_idx),
            ];
            match try_translate(&rotated, &to) {
                Some((tx, ty, tz)) => {
                    self.coords = self
                        .coords
                        .iter()
                        .map(|c| {
                            let rotated = rotate(c, rotate_idx);
                            (rotated.0 + tx, rotated.1 + ty, rotated.2 + tz)
                        })
                        .collect();
                    let rotated = rotate(&(0, 0, 0), rotate_idx);
                    self.scanner_loc = Some((rotated.0 + tx, rotated.1 + ty, rotated.2 + tz));
                }
                None => continue,
            }
        }
    }
}

fn roll(coord: &(i32, i32, i32)) -> (i32, i32, i32) {
    (coord.0, coord.2, -coord.1)
}

fn turn_cw(coord: &(i32, i32, i32)) -> (i32, i32, i32) {
    (-coord.1, coord.0, coord.2)
}

fn turn_ccw(coord: &(i32, i32, i32)) -> (i32, i32, i32) {
    (coord.1, -coord.0, coord.2)
}

fn rotate(coord: &(i32, i32, i32), nth: u32) -> (i32, i32, i32) {
    let mut curr = coord.clone();
    let mut curr_n = 0;
    if curr_n == nth {
        return curr;
    }
    for roll_idx in 0..6 {
        curr = roll(&curr);
        curr_n += 1;
        if curr_n == nth {
            return curr;
        }
        for _ in 0..3 {
            curr = if roll_idx % 2 == 0 {
                turn_cw(&curr)
            } else {
                turn_ccw(&curr)
            };
            curr_n += 1;
            if curr_n == nth {
                return curr;
            }
        }
    }
    unreachable!()
}

fn try_translate(
    from: &[(i32, i32, i32); 3],
    to: &[(i32, i32, i32); 3],
) -> Option<(i32, i32, i32)> {
    if (from[0].0 - to[0].0) == (from[1].0 - to[1].0)
        && (from[1].0 - to[1].0) == (from[2].0 - to[2].0)
        && (from[0].1 - to[0].1) == (from[1].1 - to[1].1)
        && (from[1].1 - to[1].1) == (from[2].1 - to[2].1)
        && (from[0].2 - to[0].2) == (from[1].2 - to[1].2)
        && (from[1].2 - to[1].2) == (from[2].2 - to[2].2)
    {
        Some((
            (to[0].0 - from[0].0),
            (to[0].1 - from[0].1),
            (to[0].2 - from[0].2),
        ))
    } else {
        None
    }
}

fn distances_for(
    coords: &[(i32, i32, i32)],
) -> (
    HashSet<i32>,
    HashMap<i32, (usize, usize)>,
    HashMap<usize, HashMap<i32, usize>>,
) {
    let distance_map: HashMap<i32, (usize, usize)> = coords
        .iter()
        .enumerate()
        .map(|(i, a)| {
            coords
                .iter()
                .enumerate()
                .filter_map(|(j, b)| {
                    if a == b {
                        None
                    } else {
                        Some((
                            (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2),
                            (i, j),
                        ))
                    }
                })
                .collect::<Vec<(i32, (usize, usize))>>()
        })
        .flatten()
        .collect();
    let distance_set = distance_map.keys().cloned().collect();
    let mut node_distances: HashMap<usize, HashMap<i32, usize>> = HashMap::new();
    distance_map.iter().for_each(|(dist, (a, b))| {
        (*node_distances.entry(*a).or_insert(HashMap::new())).insert(*dist, *b);
        (*node_distances.entry(*b).or_insert(HashMap::new())).insert(*dist, *a);
    });
    (distance_set, distance_map, node_distances)
}

pub fn solution_1<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let mut iter = read_lines(filename)
        .expect("failed to read input")
        .into_iter();
    let mut reports: Vec<Report> = Vec::new();
    let mut coords: Vec<(i32, i32, i32)> = Vec::new();
    iter.next();
    loop {
        match iter.next() {
            None => break,
            Some(l) => {
                let line = l.unwrap();
                if line.trim().is_empty() {
                    reports.push(Report::new(&coords));
                    coords.clear();
                    iter.next();
                    continue;
                }
                let coord: (i32, i32, i32) = line
                    .split(',')
                    .into_iter()
                    .map(|coord| coord.parse::<i32>().unwrap())
                    .collect_tuple()
                    .unwrap();
                coords.push(coord);
            }
        }
    }
    reports.push(Report::new(&coords));
    reports[0].scanner_loc = Some((0, 0, 0));
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(0);
    while let Some(next) = q.pop_front() {
        let curr = reports.get(next).unwrap().clone();
        reports.iter_mut().enumerate().for_each(|(i, next)| {
            if curr.normalize(next) {
                q.push_back(i)
            }
        });
    }
    let beacons: HashSet<_> = reports
        .into_iter()
        .map(|report| report.coords.into_iter().collect::<Vec<_>>())
        .flatten()
        .collect();
    beacons.len()
}

pub fn solution_2<P>(filename: P) -> i32
where
    P: AsRef<Path>,
{
    let mut iter = read_lines(filename)
        .expect("failed to read input")
        .into_iter();
    let mut reports: Vec<Report> = Vec::new();
    let mut coords: Vec<(i32, i32, i32)> = Vec::new();
    iter.next();
    loop {
        match iter.next() {
            None => break,
            Some(l) => {
                let line = l.unwrap();
                if line.trim().is_empty() {
                    reports.push(Report::new(&coords));
                    coords.clear();
                    iter.next();
                    continue;
                }
                let coord: (i32, i32, i32) = line
                    .split(',')
                    .into_iter()
                    .map(|coord| coord.parse::<i32>().unwrap())
                    .collect_tuple()
                    .unwrap();
                coords.push(coord);
            }
        }
    }
    reports.push(Report::new(&coords));
    reports[0].scanner_loc = Some((0, 0, 0));
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(0);
    while let Some(next) = q.pop_front() {
        let curr = reports.get(next).unwrap().clone();
        reports.iter_mut().enumerate().for_each(|(i, next)| {
            if curr.normalize(next) {
                q.push_back(i)
            }
        });
    }
    let locs: Vec<(i32, i32, i32)> = reports
        .into_iter()
        .map(|r| r.scanner_loc.unwrap())
        .collect();
    locs.iter()
        .map(|a| {
            locs.iter()
                .map(|b| ((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}
