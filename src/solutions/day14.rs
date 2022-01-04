use std::collections::{HashMap, HashSet};

fn step_through(
    counts: &mut HashMap<char, i64>,
    pairs: &mut HashMap<(char, char), i64>,
    instructions: &HashMap<(char, char), char>,
    step: usize,
) -> i64 {
    for _ in 0..step {
        let mut pairs_to_remove = HashSet::new();
        let mut pairs_to_add = HashMap::new();
        for instruction in instructions {
            if let Some(pair_count) = pairs.get(instruction.0) {
                *counts.entry(*instruction.1).or_insert(0) += pair_count;

                pairs_to_remove.insert(*instruction.0);

                let new_pair = (instruction.0 .0, *instruction.1);
                *pairs_to_add.entry(new_pair).or_insert(0 as i64) += pair_count;

                let new_pair = (*instruction.1, instruction.0 .1);
                *pairs_to_add.entry(new_pair).or_insert(0 as i64) += pair_count;
            }
        }

        for pair in pairs_to_remove {
            pairs.remove(&pair);
        }

        for (pair, count) in pairs_to_add {
            *pairs.entry(pair).or_insert(0) += count;
        }
    }

    let counts = counts.iter().map(|x| *x.1).collect::<Vec<_>>();
    *counts.iter().max().unwrap() - *counts.iter().min().unwrap()
}

pub fn solve(input: String) {
    let template = input.lines().next().unwrap().chars().collect::<Vec<_>>();
    let mut pairs = HashMap::new();
    let mut counts = HashMap::new();
    for i in 0..template.len() {
        *counts.entry(template[i]).or_insert(0 as i64) += 1;
        if i < template.len() - 1 {
            *pairs
                .entry((template[i], template[i + 1]))
                .or_insert(0 as i64) += 1;
        }
    }

    let mut instructions = HashMap::new();
    for line in input.lines() {
        if line.contains('-') {
            let split_line = line.split(" -> ").collect::<Vec<_>>();
            let pattern = split_line[0].chars().collect::<Vec<_>>();
            let pattern = (pattern[0], pattern[1]);
            let to = split_line[1].chars().next().unwrap();
            instructions.insert(pattern, to);
        }
    }

    println!(
        "Part 1: {}",
        step_through(&mut counts.clone(), &mut pairs.clone(), &instructions, 10)
    );
    println!(
        "Part 2: {}",
        step_through(&mut counts.clone(), &mut pairs.clone(), &instructions, 40)
    );
}
