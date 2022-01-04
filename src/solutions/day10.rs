fn is_opening_char(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn is_matching_close(open: char, close: char) -> bool {
    (open == '(' && close == ')')
        || (open == '[' && close == ']')
        || (open == '{' && close == '}')
        || (open == '<' && close == '>')
}

fn check_line(line: &Vec<char>) -> (Vec<char>, Option<char>) {
    let mut stack = Vec::new();
    for c in line {
        if is_opening_char(*c) {
            stack.push(*c);
        } else if stack.len() > 0 && is_matching_close(*stack.last().unwrap(), *c) {
            stack.pop();
        } else {
            return (stack, Some(*c));
        }
    }
    (stack, None)
}

fn part1(lines: &Vec<Vec<char>>) -> u64 {
    let mut total: u64 = 0;
    for line in lines {
        let (_, illegal_char) = check_line(line);
        if let Some(c) = illegal_char {
            total += match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => {
                    println!("Shouldn't get here!");
                    0
                }
            }
        }
    }
    total
}

fn part2(lines: &Vec<Vec<char>>) -> u64 {
    let mut scores = Vec::new();
    for line in lines {
        let (mut openers, illegal_char) = check_line(line);
        let mut line_score: u64 = 0;
        if let None = illegal_char {
            while openers.len() > 0 {
                line_score = line_score * 5
                    + match openers.pop().unwrap() {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => {
                            println!("Shouldn't get here!");
                            0
                        }
                    }
            }
            scores.push(line_score);
        }
    }

    let middle = scores.len() / 2;
    scores.select_nth_unstable(middle);
    scores[scores.len() / 2]
}

pub fn solve(input: String) {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}
