use itertools::Itertools;
use std::collections::HashMap;
use Ord;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn sorted(&self) -> Self {
        Line {
            start: Ord::min(self.start, self.end),
            end: Ord::max(self.start, self.end),
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn get_horizontal_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for x in self.start.x..=self.end.x {
            points.push(Point { x, y: self.start.y });
        }
        points
    }

    fn get_vertical_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for y in Ord::min(self.start.y, self.end.y)..=Ord::max(self.start.y, self.end.y) {
            points.push(Point { x: self.start.x, y });
        }
        points
    }

    fn get_slope(&self) -> i32 {
        if self.start.y < self.end.y {
            1
        } else {
            -1
        }
    }

    fn get_diagonal_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let slope = self.get_slope();
        for i in 0..=(self.end.x - self.start.x) {
            points.push(Point {
                x: self.start.x + i,
                y: self.start.y + i * slope,
            });
        }
        points
    }

    fn count_overlaps(lines: &Vec<Line>, diagonals: bool) -> u32 {
        let mut counts = HashMap::new();
        for line in lines {
            let points = if line.is_horizontal() {
                line.get_horizontal_points()
            } else if line.is_vertical() {
                line.get_vertical_points()
            } else if diagonals {
                line.get_diagonal_points()
            } else {
                Vec::new()
            };
            for point in points {
                *counts.entry(point).or_insert(0) += 1;
            }
        }
        counts.values().filter(|v| **v >= 2).count() as u32
    }
}

fn part1(lines: &Vec<Line>) -> u32 {
    Line::count_overlaps(lines, false)
}

fn part2(lines: &Vec<Line>) -> u32 {
    Line::count_overlaps(lines, true)
}

pub fn solve(input: String) {
    let lines = input
        .lines()
        .map(|line| {
            let points = line
                .split(" -> ")
                .map(|pair_str| {
                    let pair = pair_str
                        .split(',')
                        .map(|elem| elem.parse::<i32>().unwrap())
                        .collect_vec();
                    Point {
                        x: pair[0],
                        y: pair[1],
                    }
                })
                .collect_vec();
            Line {
                start: points[0],
                end: points[1],
            }
            .sorted()
        })
        .collect_vec();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}
