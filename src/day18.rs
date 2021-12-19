use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fmt::Debug,
    ops::Add,
    path::Path,
    str::FromStr,
};

use itertools::Itertools;

use crate::util::read_lines;

#[derive(Debug, Clone)]
enum Node {
    Pair(Box<Pair>),
    Lit(u32),
}

#[derive(Debug, Clone)]
struct Pair {
    left: Node,
    right: Node,
}

#[derive(Debug)]
enum ParseNode {
    Node(Node),
    Open,
}

impl FromStr for Pair {
    type Err = std::io::Error;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut stack: VecDeque<ParseNode> = VecDeque::new();
        line.chars().for_each(|c| match c {
            '[' => {
                stack.push_back(ParseNode::Open);
            }
            ',' => (),
            ']' => match (stack.pop_back().unwrap(), stack.pop_back().unwrap()) {
                (ParseNode::Node(right), ParseNode::Node(left)) => {
                    match stack.pop_back().unwrap() {
                        ParseNode::Open => {
                            stack.push_back(ParseNode::Node(Node::Pair(Box::new(Pair {
                                left: left,
                                right: right,
                            }))))
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            },
            _ => stack.push_back(ParseNode::Node(Node::Lit(c.to_digit(10).unwrap()))),
        });
        match stack.pop_back().unwrap() {
            ParseNode::Node(Node::Pair(pair)) => Ok(*pair),
            _ => unreachable!(),
        }
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = Self {
            left: Node::Pair(Box::new(self)),
            right: Node::Pair(Box::new(other)),
        };
        res.reduce();
        res
    }
}

enum Action {
    Exploded(bool, Option<u32>, Option<u32>),
    Nothing,
}

impl Pair {
    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            } else if !self.split() {
                break;
            }
        }
    }

    fn send_left(&mut self, going_down: bool, val: &u32) -> bool {
        if !going_down {
            match &mut self.right {
                Node::Lit(curr) => {
                    *curr += *val;
                    return false;
                }
                Node::Pair(p) => {
                    let cont = (*p).send_left(false, val);
                    if !cont {
                        return false;
                    }
                }
            }
        }

        match &mut self.left {
            Node::Lit(curr) => {
                *curr += *val;
                false
            }
            Node::Pair(p) => {
                let cont = (*p).send_left(false, val);
                cont
            }
        }
    }

    fn send_right(&mut self, going_down: bool, val: &u32) -> bool {
        if !going_down {
            match &mut self.left {
                Node::Lit(curr) => {
                    *curr += *val;
                    return false;
                }
                Node::Pair(p) => {
                    let cont = (*p).send_right(false, val);
                    if !cont {
                        return false;
                    }
                }
            }
        }

        match &mut self.right {
            Node::Lit(curr) => {
                *curr += *val;
                false
            }
            Node::Pair(p) => {
                let cont = (*p).send_right(false, val);
                cont
            }
        }
    }

    fn explode_helper(&mut self, depth: u32) -> Action {
        if depth == 0 {
            match (&self.left, &self.right) {
                (Node::Lit(l), Node::Lit(r)) => Action::Exploded(true, Some(*l), Some(*r)),
                _ => unreachable!(),
            }
        } else {
            match &mut self.left {
                Node::Lit(_) => (),
                Node::Pair(p) => match p.explode_helper(depth - 1) {
                    Action::Exploded(just_explode, maybe_left, maybe_right) => {
                        if just_explode {
                            self.left = Node::Lit(0);
                        }
                        let bubble_right = {
                            if let Some(right) = maybe_right {
                                if self.send_right(true, &right) {
                                    Some(right)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        };
                        return Action::Exploded(false, maybe_left, bubble_right);
                    }
                    Action::Nothing => (),
                },
            }
            match &mut self.right {
                Node::Lit(_) => Action::Nothing,
                Node::Pair(p) => match p.explode_helper(depth - 1) {
                    Action::Exploded(just_explode, maybe_left, maybe_right) => {
                        if just_explode {
                            self.right = Node::Lit(0);
                        }
                        let bubble_left = {
                            if let Some(left) = maybe_left {
                                if self.send_left(true, &left) {
                                    Some(left)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        };
                        Action::Exploded(false, bubble_left, maybe_right)
                    }
                    Action::Nothing => Action::Nothing,
                },
            }
        }
    }

    fn explode(&mut self) -> bool {
        match self.explode_helper(4) {
            Action::Nothing => false,
            Action::Exploded(_, _, _) => true,
        }
    }

    fn split(&mut self) -> bool {
        match &mut self.left {
            &mut Node::Lit(l) => {
                if l >= 10 {
                    self.left = Node::Pair(Box::new(Pair {
                        left: Node::Lit(l / 2),
                        right: Node::Lit((l + 1) / 2),
                    }));
                    return true;
                }
            }
            Node::Pair(p) => {
                if p.split() {
                    return true;
                }
            }
        }
        match &mut self.right {
            &mut Node::Lit(l) => {
                if l >= 10 {
                    self.right = Node::Pair(Box::new(Pair {
                        left: Node::Lit(l / 2),
                        right: Node::Lit((l + 1) / 2),
                    }));
                    return true;
                }
            }
            Node::Pair(p) => {
                if p.split() {
                    return true;
                }
            }
        }
        false
    }

    fn magnitude(&self) -> u64 {
        (self.left.magnitude() as u64) * 3 + (self.right.magnitude() as u64) * 2
    }
}

impl Node {
    fn magnitude(&self) -> u64 {
        match self {
            Node::Pair(p) => p.magnitude(),
            Node::Lit(l) => *l as u64,
        }
    }
}

pub fn solution_1<P>(filename: P) -> u64
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename).expect("failed to read input");
    let mut pairs = lines
        .into_iter()
        .map(|line| line.unwrap().parse::<Pair>().unwrap());
    let first = pairs.next().unwrap();
    let res = pairs.fold(first, |acc, new| acc + new);
    println!("{:#?}", res);
    res.magnitude()
}

pub fn solution_2<P>(filename: P) -> u64
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename).expect("failed to read input");
    let pairs: Vec<_> = lines
        .into_iter()
        .map(|line| line.unwrap().parse::<Pair>().unwrap())
        .collect();
    let mut max = 0;
    for i in 0..pairs.len() {
        for j in 0..pairs.len() {
            if i == j {
                continue;
            }
            max = std::cmp::max((pairs[i].clone() + pairs[j].clone()).magnitude(), max)
        }
    }
    max
}
