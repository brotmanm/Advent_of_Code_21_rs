use itertools::Itertools;

fn count_in_column(bits: &Vec<Vec<u32>>, col: usize, value: u32) -> usize {
    bits.iter()
        .flatten()
        .skip(col)
        .step_by(bits[0].len())
        .filter(|elem| **elem == value)
        .count()
}

fn binary_to_decimal(binary: &[u32]) -> u32 {
    binary.iter().fold(0, |acc, b| (acc << 1) + *b)
}

fn get_rating(bits: &Vec<Vec<u32>>, most_common: bool) -> u32 {
    let mut bits = bits.clone();
    for i in 0..bits[0].len() {
        let zeros = count_in_column(&bits, i, 0);
        let ones = count_in_column(&bits, i, 1);
        if most_common {
            bits = bits
                .into_iter()
                .filter(|line| line[i] == (ones >= zeros) as u32)
                .collect_vec();
        } else {
            bits = bits
                .into_iter()
                .filter(|line| line[i] == (ones < zeros) as u32)
                .collect_vec();
        }
        if bits.len() == 1 {
            break;
        }
    }
    binary_to_decimal(&bits[0])
}

fn part1(bits: &Vec<Vec<u32>>) -> u32 {
    let mut gamma = Vec::new();
    for i in 0..bits[0].len() {
        gamma.push((count_in_column(bits, i, 1) >= count_in_column(bits, i, 0)) as u32);
    }
    let eps = gamma.iter().map(|b| 1 - *b).collect_vec();

    binary_to_decimal(&gamma) * binary_to_decimal(&eps)
}

fn part2(bits: &Vec<Vec<u32>>) -> u32 {
    get_rating(bits, true) * get_rating(bits, false)
}

pub fn solve(input: String) {
    let mut bits = Vec::new();
    for line in input.lines() {
        let mut bitline = Vec::new();
        for c in line.chars() {
            bitline.push(c.to_digit(10).unwrap());
        }
        bits.push(bitline);
    }

    println!("Part 1: {}", part1(&bits));
    println!("Part 2: {}", part2(&bits));
}
