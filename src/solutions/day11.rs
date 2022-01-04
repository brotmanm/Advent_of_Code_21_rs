fn count_flashes(energies: &mut Vec<Vec<u32>>, i: i32, j: i32) -> u64 {
    if i >= 0 && i < energies.len() as i32 && j >= 0 && j < energies[0].len() as i32 {
        let e = &mut energies[i as usize][j as usize];
        if *e > 0 && *e < 10 {
            *e += 1;
        }
        if *e == 10 {
            *e = 0;
            let mut flashes = 1;
            for r in (i - 1)..=(i + 1) {
                for c in (j - 1)..=(j + 1) {
                    if r != i || c != j {
                        flashes += count_flashes(energies, r, c);
                    }
                }
            }
            flashes
        } else {
            0
        }
    } else {
        0
    }
}

fn part1(energies: &mut Vec<Vec<u32>>, steps: u32) -> u64 {
    let mut total: u64 = 0;
    for _ in 0..steps {
        for i in 0..energies.len() {
            for j in 0..energies[0].len() {
                energies[i][j] += 1;
            }
        }

        for i in 0..energies.len() {
            for j in 0..energies[0].len() {
                if energies[i][j] == 10 {
                    total += count_flashes(energies, i as i32, j as i32);
                }
            }
        }
    }
    total
}

fn part2(energies: &mut Vec<Vec<u32>>) -> u32 {
    let mut step = 0;
    loop {
        step += 1;
        for i in 0..energies.len() {
            for j in 0..energies[0].len() {
                energies[i][j] += 1;
            }
        }

        let mut total: u64 = 0;
        for i in 0..energies.len() {
            for j in 0..energies[0].len() {
                if energies[i][j] == 10 {
                    total += count_flashes(energies, i as i32, j as i32);
                }
            }
        }
        if total == (energies.len() * energies[0].len()) as u64 {
            return step;
        }
    }
}

pub fn solve(input: String) {
    let energies = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&mut energies.clone(), 100));
    println!("Part 2: {}", part2(&mut energies.clone()));
}
