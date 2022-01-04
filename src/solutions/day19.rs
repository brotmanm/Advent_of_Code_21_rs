use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Debug)]
struct Scanner {
    beacons: HashSet<Point>,
}

impl Point {
    fn orient(&self, orientation: &[usize], flip: &[usize]) -> Self {
        let as_vec = vec![self.x, self.y, self.z];
        let mut new_vec = vec![0; 3];
        for i in 0..3 {
            new_vec[i] = as_vec[orientation[i]];
            if flip[i] == 1 {
                new_vec[i] *= -1;
            }
        }
        Point {
            x: new_vec[0],
            y: new_vec[1],
            z: new_vec[2],
        }
    }

    fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Scanner {
    fn try_find_overlap(&self, other: &Scanner) -> Option<(Self, Point)> {
        for axis_order in (0..3).permutations(3) {
            for axis_flips in 0..8 {
                let flip = [axis_flips & 1, (axis_flips >> 1) & 1, (axis_flips >> 2) & 1];
                let other_points_oriented = other
                    .beacons
                    .iter()
                    .map(|p| p.orient(&axis_order, &flip))
                    .collect::<HashSet<_>>();
                for our_point in &self.beacons {
                    for other_point in &other_points_oriented {
                        let diff = *other_point - *our_point;
                        let other_points_moved = other_points_oriented
                            .iter()
                            .map(|p| *p - diff)
                            .collect::<HashSet<_>>();
                        if self.beacons.intersection(&other_points_moved).count() >= 12 {
                            return Some((
                                Scanner {
                                    beacons: other_points_moved,
                                },
                                -diff,
                            ));
                        }
                    }
                }
            }
        }
        None
    }

    fn merge(&mut self, others: &mut Vec<Scanner>) -> Vec<Point> {
        let mut scanner_positions = Vec::new();
        while !others.is_empty() {
            let mut removed_one = false;
            for i in 0..others.len() {
                if let Some((new_scanner, pos)) = self.try_find_overlap(&others[i]) {
                    removed_one = true;
                    others.swap_remove(i);
                    self.beacons = self
                        .beacons
                        .union(&new_scanner.beacons)
                        .map(|p| *p)
                        .collect();
                    scanner_positions.push(pos);
                    break;
                }
            }
            if !removed_one {
                panic!();
            }
        }
        scanner_positions
    }
}

fn part1(scanner: &Scanner) -> usize {
    scanner.beacons.len()
}

fn part2(scanners: &Vec<Point>) -> i32 {
    scanners
        .iter()
        .cartesian_product(scanners.iter())
        .map(|(p1, p2)| p1.manhattan_distance(p2))
        .max()
        .unwrap()
}

pub fn solve(input: String) {
    let input = input;
    let mut scanners = Vec::new();

    let mut current_beacons = HashSet::new();
    for line in input.lines() {
        if line.contains("scanner") {
            current_beacons = HashSet::new();
        } else if line.is_empty() {
            scanners.push(Scanner {
                beacons: current_beacons.clone(),
            });
        } else {
            let beacon = line
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            current_beacons.insert(Point {
                x: beacon[0],
                y: beacon[1],
                z: beacon[2],
            });
        }
    }
    scanners.push(Scanner {
        beacons: current_beacons.clone(),
    });

    let mut scanners_located = Scanner {
        beacons: scanners.remove(0).beacons,
    };
    let scanner_positions = scanners_located.merge(&mut scanners);

    println!("Part 1: {}", part1(&scanners_located));
    println!("Part 2: {}", part2(&scanner_positions));
}
