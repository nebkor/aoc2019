use std::cmp::{max, min, Eq, PartialEq};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub steps: u32,
}

impl Point {
    pub fn dist(self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

fn get_input() -> Vec<Vec<String>> {
    let wires: Vec<_> = include_str!("input").trim().split('\n').collect();
    let mut ret: Vec<Vec<String>> = Vec::new();
    for wire in wires {
        let w = wire.split(',').map(|x| x.trim().to_owned()).collect();
        ret.push(w);
    }
    ret
}

fn parse_input(input: &[Vec<String>]) -> Vec<HashSet<Point>> {
    let mut ret = Vec::with_capacity(input.len());

    for wire in input {
        let mut prev_x: i32 = 0;
        let mut prev_y: i32 = 0;
        let mut steps: u32 = 0;

        let mut points = HashSet::<Point>::new();
        for d in wire.iter() {
            let new_y: i32;
            let new_x: i32;
            let (d, n) = d.split_at(1);
            let n = n.parse::<i32>().unwrap();
            match d {
                "D" => {
                    new_y = prev_y - n;
                    for y in new_y..prev_y {
                        steps += 1;
                        let p = Point {
                            x: prev_x,
                            y,
                            steps,
                        };
                        points.insert(p);
                    }
                    prev_y = new_y;
                }
                "U" => {
                    new_y = prev_y + n;
                    for y in prev_y..=new_y {
                        steps += 1;
                        let p = Point {
                            x: prev_x,
                            y,
                            steps,
                        };
                        points.insert(p);
                    }
                    prev_y = new_y;
                }
                "L" => {
                    new_x = prev_x - n;
                    for x in new_x..prev_x {
                        steps += 1;
                        let p = Point {
                            x,
                            y: prev_y,
                            steps,
                        };
                        points.insert(p);
                    }
                    prev_x = new_x;
                }
                "R" => {
                    new_x = prev_x + n;
                    for x in prev_x..=new_x {
                        steps += 1;
                        let p = Point {
                            x,
                            y: prev_y,
                            steps,
                        };
                        points.insert(p);
                    }
                    prev_x = new_x;
                }
                _ => {}
            }
        }
        ret.push(points);
    }

    ret
}

fn main() {
    let input = get_input();
    let input = parse_input(&input);

    // TODO: generalize on number of wires
    assert!(&input.len() == &(2 as usize));

    let mut intersection: HashSet<_> = input[0].intersection(&input[1]).collect();
    let _ = intersection.remove(
        &(Point {
            x: 0,
            y: 0,
            steps: std::u32::MAX, // steps are not considered part of the identity, so value is irrelevant
        }),
    );

    for p in intersection.iter() {
        dbg!(p);
    }

    let p = intersection.iter().map(|p| p.dist()).min();
    println!("part 1: {}", p.unwrap());
}
