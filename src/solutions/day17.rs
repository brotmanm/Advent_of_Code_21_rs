use std::collections::HashSet;
use Ord;

fn part1(bounds: &[i32]) -> i32 {
    let y = bounds[2];
    assert!(y < 0);

    let v = -y - 1;
    let max_height = v * (v + 1) / 2;
    max_height
}

fn part2(bounds: &[i32]) -> usize {
    let mut velocities = HashSet::new();

    for vx in 0..=(bounds[1]) {
        for vy in (bounds[2])..=(-bounds[2] - 1) {
            for step in 1.. {
                let y = step * vy - step * (step - 1) / 2;
                let x_steps = Ord::min(step, vx);
                let x = x_steps * vx - x_steps * (x_steps - 1) / 2;
                if x >= bounds[0] && x <= bounds[1] && y >= bounds[2] && y <= bounds[3] {
                    velocities.insert((vx, vy));
                } else if x > bounds[1] || y < bounds[2] {
                    break;
                }
            }
        }
    }
    velocities.len()
}

pub fn solve(input: String) {
    let bounds = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&bounds));
    println!("Part 2: {}", part2(&bounds));
}
