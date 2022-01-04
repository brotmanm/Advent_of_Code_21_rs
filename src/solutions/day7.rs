use std::cmp;

fn part1(positions: &Vec<i64>) -> i64 {
    let mut positions = positions.clone();
    positions.sort();
    let median = positions[positions.len() / 2];

    positions.iter().fold(0, |acc, p| acc + (p - median).abs())
}

fn part2(positions: &Vec<i64>) -> i64 {
    let mean = positions.iter().sum::<i64>() as f64 / positions.len() as f64;
    let mean_rounded_up = mean.ceil() as i64;

    let find_total_cost = |move_to: i64| {
        positions.iter().fold(0, |acc, p| {
            acc + {
                let n = (p - move_to).abs();
                n * (n + 1) / 2
            }
        })
    };

    cmp::min(
        find_total_cost(mean_rounded_up),
        find_total_cost(mean_rounded_up - 1),
    )
}

pub fn solve(input: String) {
    let positions: Vec<i64> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|p| p.parse::<i64>().unwrap())
        .collect();

    println!("Part 1: {}", part1(&positions));
    println!("Part 2: {}", part2(&positions));
}
