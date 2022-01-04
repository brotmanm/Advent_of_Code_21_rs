enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn part1(commands: &[Command]) -> u32 {
    let (mut x, mut y) = (0, 0);
    for command in commands {
        match command {
            Command::Forward(val) => x += val,
            Command::Down(val) => y += val,
            Command::Up(val) => y -= val,
        }
    }
    x * y
}

fn part2(commands: &[Command]) -> u32 {
    let (mut x, mut y, mut aim) = (0, 0, 0);
    for command in commands {
        match command {
            Command::Forward(val) => {
                x += val;
                y += aim * val;
            }
            Command::Down(val) => aim += val,
            Command::Up(val) => aim -= val,
        }
    }
    x * y
}

pub fn solve(input: String) {
    let mut commands = Vec::new();
    for line in input.lines() {
        let val = line.split(' ').last().unwrap().parse::<u32>().unwrap();
        if line.contains("forward") {
            commands.push(Command::Forward(val));
        } else if line.contains("down") {
            commands.push(Command::Down(val));
        } else if line.contains("up") {
            commands.push(Command::Up(val));
        }
    }

    println!("Part 1: {}", part1(&commands));
    println!("Part 2: {}", part2(&commands));
}
