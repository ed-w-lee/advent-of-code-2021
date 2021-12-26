use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::Path,
};

use itertools::Itertools;

use crate::util::read_lines;

type Range = (i32, i32);
type Cube = (Range, Range, Range);

fn parse_line(line: String) -> (bool, Cube) {
    let mut split = line.split_whitespace();
    let is_on = match split.next().unwrap() {
        "on" => true,
        "off" => false,
        _ => unreachable!(),
    };
    let ((xmin, xmax), (ymin, ymax), (zmin, zmax)) = split
        .next()
        .unwrap()
        .split(',')
        .map(|range_str| {
            let range = range_str.split('=').nth(1).unwrap();
            range
                .split("..")
                .map(|bound| bound.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
    (is_on, ((xmin, xmax), (ymin, ymax), (zmin, zmax)))
}

fn clamp<T>(v: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if v < min {
        min
    } else if max < v {
        max
    } else {
        v
    }
}

pub fn solution_1<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    const BOUND: i32 = 50;
    let lines = read_lines(filename).expect("failed to read input");
    let coords: HashSet<(i32, i32, i32)> = lines
        .into_iter()
        .map(|line| parse_line(line.unwrap()))
        .fold(HashSet::new(), |mut acc, input| {
            let (is_on, ((xmin, xmax), (ymin, ymax), (zmin, zmax))) = input;
            if xmin > BOUND || ymin > BOUND || zmin > BOUND {
                return acc;
            } else if xmax < -BOUND || ymax < -BOUND || zmax < -BOUND {
                return acc;
            }
            let xmin = clamp(xmin, -BOUND, BOUND);
            let xmax = clamp(xmax, -BOUND, BOUND);
            let ymin = clamp(ymin, -BOUND, BOUND);
            let ymax = clamp(ymax, -BOUND, BOUND);
            let zmin = clamp(zmin, -BOUND, BOUND);
            let zmax = clamp(zmax, -BOUND, BOUND);
            println!(
                "{} {}..{} {}..{} {}..{}",
                is_on, xmin, xmax, ymin, ymax, zmin, zmax
            );
            (xmin..(xmax + 1))
                .cartesian_product(ymin..(ymax + 1))
                .cartesian_product(zmin..(zmax + 1))
                .into_iter()
                .for_each(|((x, y), z)| {
                    if is_on {
                        acc.insert((x, y, z));
                    } else {
                        acc.remove(&(x, y, z));
                    }
                });
            acc
        });
    coords.len()
}

struct Cuboids {
    // allows querying from cube i -> cube & is_on
    cuboids: Vec<(bool, Cube)>,
    // allows ordered traversal from x position -> (cube idx, is_min_val)
    x_ranges: Vec<(i32, (usize, bool))>,
}

struct Squares {
    // cube_idx -> (is_on, y range, z range)
    cubes: HashMap<usize, (bool, Range, Range)>,
    // sorted (y_pos, cube idx) -> is_range_start
    y_ranges: BTreeMap<(i32, usize), bool>,
    // // sorted (z_pos, cube idx) -> is_range_start
    // z_ranges: BTreeMap<(i32, usize), bool>,
}

impl Squares {
    fn new() -> Self {
        Self {
            cubes: HashMap::new(),
            y_ranges: BTreeMap::new(),
            // z_ranges: BTreeMap::new(),
        }
    }

    fn add_x(&mut self, cube_idx: usize, is_on: bool, y_range: Range, z_range: Range) {
        self.cubes.insert(cube_idx, (is_on, y_range, z_range));
        self.y_ranges.insert((y_range.0, cube_idx), true);
        self.y_ranges.insert((y_range.1 + 1, cube_idx), false);
    }

    fn remove_x(&mut self, cube_idx: usize) {
        let (_, y_range, _) = self.cubes.get(&cube_idx).unwrap();
        self.y_ranges.remove(&(y_range.0, cube_idx));
        self.y_ranges.remove(&(y_range.1 + 1, cube_idx));
    }

    fn current_area(&self) -> usize {
        // (z_pos, cube_idx) -> (is_start_range, is_on)
        let mut active_z: BTreeMap<(i32, usize), (bool, bool)> = BTreeMap::new();
        let y_status = self
            .y_ranges
            .iter()
            .map(|((y_pos, cube_idx), is_range_start)| {
                let (is_on, _, z_range) = self.cubes.get(&cube_idx).unwrap();
                if *is_range_start {
                    active_z.insert((z_range.0, *cube_idx), (true, *is_on));
                    active_z.insert((z_range.1 + 1, *cube_idx), (false, *is_on));
                } else {
                    active_z.remove(&(z_range.0, *cube_idx));
                    active_z.remove(&(z_range.1 + 1, *cube_idx));
                }

                // sorted cube_idx -> is_on
                let mut layer_state: BTreeMap<usize, bool> = BTreeMap::from([(0, false)]);
                let z_status: Vec<_> = active_z
                    .iter()
                    .map(|((z, cube_idx), (is_start, is_on))| {
                        if *is_start {
                            layer_state.insert(*cube_idx, *is_on);
                        } else {
                            layer_state.remove(cube_idx);
                        }
                        let (_, is_final_on) = layer_state.iter().next_back().unwrap();
                        (z, *is_final_on)
                    })
                    .collect();
                let mut line_area = 0;
                for i in 1..(z_status.len()) {
                    let (z_start, is_on) = z_status[i - 1];
                    let (z_end, _) = z_status[i];
                    if is_on {
                        line_area += z_end - z_start;
                    }
                }
                (*y_pos, line_area)
            })
            .collect::<Vec<_>>();
        let mut area = 0;
        for i in 1..y_status.len() {
            let (y_start, line_area) = y_status[i - 1];
            let (y_end, _) = y_status[i];
            area += (y_end - y_start) as usize * line_area as usize;
        }

        area
    }
}

impl Cuboids {
    fn new(cuboids: Vec<(bool, Cube)>) -> Self {
        let x_ranges: Vec<(i32, (usize, bool))> = cuboids
            .iter()
            .enumerate()
            .map(|(i, (_, ((x_min, x_max), _, _)))| {
                [(*x_min, (i + 1, true)), (x_max + 1, (i + 1, false))]
            })
            .flatten()
            .sorted()
            .collect();
        Self { cuboids, x_ranges }
    }

    fn count_num_on(self) -> usize {
        let mut squares = Squares::new();
        let x_status = self
            .x_ranges
            .iter()
            .map(|(x, (cube_idx, is_start))| {
                let (is_on, (_, y_range, z_range)) = self.cuboids[*cube_idx - 1];
                if *is_start {
                    squares.add_x(*cube_idx, is_on, y_range, z_range);
                } else {
                    squares.remove_x(*cube_idx);
                }
                let area = squares.current_area();
                (*x, area)
            })
            .collect::<Vec<_>>();

        let mut volume = 0;
        for i in 1..x_status.len() {
            let (x_start, area) = x_status[i - 1];
            let (x_end, _) = x_status[i];
            volume += (x_end - x_start) as usize * area;
        }

        volume
    }
}

pub fn solution_2<P>(filename: P) -> usize
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename).expect("failed to read input");
    let coords = lines
        .into_iter()
        .map(|line| parse_line(line.unwrap()))
        .collect::<Vec<_>>();
    let cubes = Cuboids::new(coords);
    cubes.count_num_on()
}
