use std::collections::BinaryHeap;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    r: usize,
    c: usize,
}

impl Pos {
    fn get_adjacent(&self, max_width: usize, max_height: usize) -> Vec<Pos> {
        let mut adjacents: Vec<Pos> = Vec::new();
        if self.r > 0 {
            adjacents.push(Pos {
                r: self.r - 1,
                c: self.c,
            })
        }
        if self.r < max_height - 1 {
            adjacents.push(Pos {
                r: self.r + 1,
                c: self.c,
            })
        }
        if self.c > 0 {
            adjacents.push(Pos {
                r: self.r,
                c: self.c - 1,
            })
        }
        if self.c < max_width - 1 {
            adjacents.push(Pos {
                r: self.r,
                c: self.c + 1,
            })
        }
        adjacents
    }
}

impl<T> Index<Pos> for Vec<Vec<T>> {
    type Output = T;
    fn index(&self, index: Pos) -> &T {
        &self[index.r][index.c]
    }
}

impl<T> IndexMut<Pos> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Pos) -> &mut T {
        &mut self[index.r][index.c]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    neg_cost: i64,
    pos: Pos,
}

// Inspired by https://doc.rust-lang.org/std/collections/binary_heap/index.html
fn shortest_paths(grid: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut distances = vec![vec![i64::MAX; grid[0].len()]; grid.len()];
    let mut closest = BinaryHeap::new();

    let start = Pos { r: 0, c: 0 };
    let end = Pos {
        r: grid.len() - 1,
        c: grid[0].len() - 1,
    };

    distances[start] = 0;
    closest.push(Node {
        neg_cost: 0,
        pos: start,
    });

    while let Some(node) = closest.pop() {
        if distances[node.pos] < -node.neg_cost {
            continue;
        }

        if node.pos == end {
            return distances;
        }

        for adj in node.pos.get_adjacent(grid[0].len(), grid.len()) {
            let new_cost = distances[node.pos] + grid[adj];
            if new_cost < distances[adj] {
                distances[adj] = new_cost;
                closest.push(Node {
                    neg_cost: -new_cost,
                    pos: adj,
                });
            }
        }
    }

    distances
}

fn part1(grid: &Vec<Vec<i64>>) -> i64 {
    let shortest = shortest_paths(grid);
    shortest[grid.len() - 1][grid[0].len() - 1]
}

fn part2(grid: &Vec<Vec<i64>>) -> i64 {
    let mut expanded = vec![vec![0; grid[0].len() * 5]; grid.len() * 5];
    for r in 0..expanded.len() {
        for c in 0..expanded[0].len() {
            expanded[r][c] = {
                let modified_risk = grid[r % grid.len()][c % grid[0].len()]
                    + (r / grid.len()) as i64
                    + (c / grid[0].len()) as i64;
                if modified_risk > 9 {
                    modified_risk - 9
                } else {
                    modified_risk
                }
            }
        }
    }

    let shortest = shortest_paths(&expanded);
    shortest[expanded.len() - 1][expanded[0].len() - 1]
}

pub fn solve(input: String) {
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<_>>(),
        )
    }

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
