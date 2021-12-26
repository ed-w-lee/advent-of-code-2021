use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::Path,
};

use itertools::Itertools;

use crate::util::read_lines;

type Coord = (i32, i32, i32);
type DistanceSet = HashSet<i32>;
type EdgeDistanceForNodes = HashMap<i32, (usize, usize)>;
type NodeToDistanceToNode = HashMap<usize, HashMap<i32, usize>>;

#[derive(Debug, Clone)]
struct Report {
    coords: Vec<Coord>,
    distance_set: DistanceSet,
    distance_map: EdgeDistanceForNodes,
    node_distances: NodeToDistanceToNode,
    scanner_loc: Option<Coord>,
}

impl Report {
    fn new(coords: &[Coord]) -> Self {
        let (distance_set, distance_map, node_distances) = distances_for(coords);
        Self {
            coords: coords.iter().copied().collect(),
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
        if other.scanner_loc.is_some() {
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
            .get(self_coord1)
            .unwrap()
            .keys()
            .collect();
        let cand_distances: HashSet<_> = other
            .node_distances
            .get(cand_coord1)
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
                .find(|(dist, idx)| s2 != *idx && other.distance_set.contains(*dist))
                .unwrap()
        };
        let o1_distances = other.node_distances.get(o1).unwrap();
        let o3 = o1_distances.get(dist3).unwrap();

        let from = [other.coords[*o1], other.coords[*o2], other.coords[*o3]];
        let to = [self.coords[*s1], self.coords[*s2], self.coords[*s3]];
        other.transform(from, to);
        true
    }

    fn transform(&mut self, from: [Coord; 3], to: [Coord; 3]) {
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

fn roll(coord: &Coord) -> Coord {
    (coord.0, coord.2, -coord.1)
}

fn turn_cw(coord: &Coord) -> Coord {
    (-coord.1, coord.0, coord.2)
}

fn turn_ccw(coord: &Coord) -> Coord {
    (coord.1, -coord.0, coord.2)
}

fn rotate(coord: &Coord, nth: u32) -> Coord {
    let mut curr = *coord;
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

fn try_translate(from: &[Coord; 3], to: &[Coord; 3]) -> Option<Coord> {
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

fn distances_for(coords: &[Coord]) -> (DistanceSet, EdgeDistanceForNodes, NodeToDistanceToNode) {
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
        (*node_distances.entry(*a).or_insert_with(HashMap::new)).insert(*dist, *b);
        (*node_distances.entry(*b).or_insert_with(HashMap::new)).insert(*dist, *a);
    });
    (distance_set, distance_map, node_distances)
}

pub fn solution_1<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let mut iter = read_lines(filename).expect("failed to read input");
    let mut reports: Vec<Report> = Vec::new();
    let mut coords: Vec<Coord> = Vec::new();
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
                let coord: Coord = line
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
    let mut iter = read_lines(filename).expect("failed to read input");
    let mut reports: Vec<Report> = Vec::new();
    let mut coords: Vec<Coord> = Vec::new();
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
                let coord: Coord = line
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
    let locs: Vec<Coord> = reports
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
