use itertools::Itertools;

fn part1(numbers: &[u32]) -> u32 {
    let mut total = 0;
    for i in 1..numbers.len() {
        if numbers[i] > numbers[i - 1] {
            total += 1;
        }
    }
    total
}

fn part2(numbers: &[u32]) -> u32 {
    let mut total = 0;
    for i in 0..numbers.len() - 3 {
        if numbers[i..i + 3].iter().sum::<u32>() < numbers[i + 1..i + 4].iter().sum::<u32>() {
            total += 1;
        }
    }
    total
}

pub fn solve(input: String) {
    let numbers = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect_vec();

    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}
