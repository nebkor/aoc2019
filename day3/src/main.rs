use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub steps: usize,
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
        let mut steps: usize = 0;

        let mut points = HashSet::<Point>::new();
        for d in wire.iter() {
            let new_y: i32;
            let new_x: i32;
            let (d, n) = d.split_at(1);
            let n = n.parse::<usize>().unwrap();
            match d {
                "D" => {
                    new_y = prev_y - n as i32;
                    for (i, y) in (new_y..prev_y).enumerate() {
                        let p = Point {
                            x: prev_x,
                            y,
                            steps: steps + n - i,
                        };
                        points.insert(p);
                    }
                    steps += n;
                    prev_y = new_y;
                }
                "U" => {
                    new_y = prev_y + n as i32;
                    for y in (prev_y + 1)..=new_y {
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
                    new_x = prev_x - n as i32;
                    for (i, x) in (new_x..prev_x).enumerate() {
                        let p = Point {
                            x,
                            y: prev_y,
                            steps: steps + n - i,
                        };
                        points.insert(p);
                    }
                    steps += n;
                    prev_x = new_x;
                }
                "R" => {
                    new_x = prev_x + n as i32;
                    for x in (prev_x + 1)..=new_x {
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

    let origin = Point::default();

    let mut intersections: HashSet<Point> = HashSet::new();

    let w1 = &input[0];
    let w2 = &input[1];

    for p1 in w1 {
        if w2.contains(p1) {
            let p2 = w2.get(p1).unwrap();
            let intersection = Point {
                x: p1.x,
                y: p1.y,
                steps: p1.steps + p2.steps,
            };
            intersections.insert(intersection);
        }
    }

    let _ = intersections.remove(&origin);

    let part2 = intersections.iter().map(|p| p.steps).min();
    println!("part 2: {}", part2.unwrap());
}
