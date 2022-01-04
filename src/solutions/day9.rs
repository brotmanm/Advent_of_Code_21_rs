fn find_low_points(layout: &Vec<Vec<u64>>) -> Vec<Vec<bool>> {
    let lower = |height: u64, adj: (i32, i32)| {
        if adj.0 >= 0 && adj.0 < layout.len() as i32 && adj.1 >= 0 && adj.1 < layout[0].len() as i32
        {
            height < layout[adj.0 as usize][adj.1 as usize]
        } else {
            true
        }
    };

    let mut low_points = vec![vec![false; layout[0].len()]; layout.len()];

    for r in 0..layout.len() as i32 {
        for c in 0..layout[r as usize].len() as i32 {
            let height = layout[r as usize][c as usize];
            if lower(height, (r - 1, c))
                && lower(height, (r + 1, c))
                && lower(height, (r, c - 1))
                && lower(height, (r, c + 1))
            {
                low_points[r as usize][c as usize] = true;
            }
        }
    }

    low_points
}

fn part1(layout: &Vec<Vec<u64>>) -> u64 {
    let low_points = find_low_points(layout);

    let mut sum_of_risk_levels = 0;
    for r in 0..layout.len() {
        for c in 0..layout[r].len() {
            if low_points[r][c] {
                sum_of_risk_levels += 1 + layout[r][c];
            }
        }
    }
    sum_of_risk_levels
}

fn should_recurse_on(
    than: u64,
    pos: (i32, i32),
    layout: &Vec<Vec<u64>>,
    found: &Vec<Vec<bool>>,
) -> bool {
    pos.0 >= 0
        && pos.0 < layout.len() as i32
        && pos.1 >= 0
        && pos.1 < layout[0].len() as i32
        && !found[pos.0 as usize][pos.1 as usize]
        && layout[pos.0 as usize][pos.1 as usize] > than
        && layout[pos.0 as usize][pos.1 as usize] < 9
}

fn basin_size(from: (i32, i32), layout: &Vec<Vec<u64>>, found: &mut Vec<Vec<bool>>) -> u64 {
    if found[from.0 as usize][from.1 as usize] {
        return 0;
    }
    found[from.0 as usize][from.1 as usize] = true;

    let mut total = 1;
    let cur_height = layout[from.0 as usize][from.1 as usize];
    if should_recurse_on(cur_height, (from.0 - 1, from.1), layout, found) {
        total += basin_size((from.0 - 1, from.1), layout, found);
    }
    if should_recurse_on(cur_height, (from.0 + 1, from.1), layout, found) {
        total += basin_size((from.0 + 1, from.1), layout, found);
    }
    if should_recurse_on(cur_height, (from.0, from.1 - 1), layout, found) {
        total += basin_size((from.0, from.1 - 1), layout, found);
    }
    if should_recurse_on(cur_height, (from.0, from.1 + 1), layout, found) {
        total += basin_size((from.0, from.1 + 1), layout, found);
    }
    total
}

fn part2(layout: &Vec<Vec<u64>>) -> u64 {
    let low_points = find_low_points(layout);
    let mut maximums = [0 as u64; 3];

    for r in 0..layout.len() {
        for c in 0..layout[r].len() {
            if low_points[r][c] {
                let mut basin = vec![vec![false; low_points[0].len()]; low_points.len()];
                let size = basin_size((r as i32, c as i32), layout, &mut basin);
                if size > maximums[0] {
                    maximums[0] = size;
                    maximums.sort();
                }
            }
        }
    }
    maximums[0] * maximums[1] * maximums[2]
}

pub fn solve(input: String) {
    let mut positions =
        vec![vec![0 as u64; input.lines().next().unwrap().len()]; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            positions[i][j] = c.to_digit(10).unwrap() as u64;
        }
    }

    println!("Part 1: {}", part1(&positions));
    println!("Part 2: {}", part2(&positions));
}
