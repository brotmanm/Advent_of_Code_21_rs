use itertools::Itertools;

fn count_after_days(timers: &Vec<usize>, days: usize) -> u64 {
    let mut dp = vec![vec![0 as u64; days + 1]; 9];
    for time in timers {
        dp[*time][0] = dp[*time][0] + 1;
    }

    for day in 1..=days {
        for time in 0..=8 {
            if time == 6 {
                dp[time][day] = dp[7][day - 1] + dp[0][day - 1];
            } else {
                dp[time][day] = dp[(time + 1) % 9][day - 1]
            }
        }
    }

    let mut total = 0;
    for time in 0..=8 {
        total += dp[time][days];
    }
    total
}

fn part1(timers: &Vec<usize>) -> u64 {
    count_after_days(timers, 80)
}

fn part2(timers: &Vec<usize>) -> u64 {
    count_after_days(timers, 256)
}

pub fn solve(input: String) {
    let timers = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|elem| elem.parse::<usize>().unwrap())
        .collect_vec();

    println!("Part 1: {}", part1(&timers));
    println!("Part 2: {}", part2(&timers));
}
