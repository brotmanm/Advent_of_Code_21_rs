use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Instruction {
    X(i32),
    Y(i32),
}

fn fold(points: HashSet<(i32, i32)>, instruction: Instruction) -> HashSet<(i32, i32)> {
    let mut new_points = HashSet::new();
    for point in points {
        match instruction {
            Instruction::X(fold) => {
                let folded_x = fold - (point.0 - fold).abs();
                new_points.insert((folded_x, point.1));
            }
            Instruction::Y(fold) => {
                let folded_y = fold - (point.1 - fold).abs();
                new_points.insert((point.0, folded_y));
            }
        }
    }
    new_points
}

fn part1(points: HashSet<(i32, i32)>, instructions: &Vec<Instruction>) -> u64 {
    let new_points = fold(points, instructions[0]);
    new_points.len() as u64
}

fn part2(points: HashSet<(i32, i32)>, instructions: &Vec<Instruction>) {
    let mut cur_points = points;
    for instruction in instructions {
        cur_points = fold(cur_points, *instruction);
    }

    let max_x = cur_points.iter().map(|p| p.0).max().unwrap();
    let max_y = cur_points.iter().map(|p| p.1).max().unwrap();
    for r in 0..=max_y {
        for c in 0..=max_x {
            if cur_points.contains(&(c, r)) {
                print!("██");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

pub fn solve(input: String) {
    let mut points = HashSet::new();
    let mut instructions = Vec::new();
    for line in input.lines() {
        if line.contains(',') {
            let mut coords = line.split(',').map(|x| x.parse::<i32>().unwrap());
            points.insert((coords.next().unwrap(), coords.next().unwrap()));
        } else if line.contains('x') {
            instructions.push(Instruction::X(
                line.split('=').last().unwrap().parse::<i32>().unwrap(),
            ));
        } else if line.contains('y') {
            instructions.push(Instruction::Y(
                line.split('=').last().unwrap().parse::<i32>().unwrap(),
            ));
        }
    }

    println!("Part 1: {}", part1(points.clone(), &instructions));
    part2(points.clone(), &instructions);
}
