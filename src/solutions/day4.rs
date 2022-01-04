use itertools::Itertools;

#[derive(Clone)]
struct Board {
    numbers: Vec<Vec<(u32, bool)>>,
    won: bool,
}

impl Board {
    fn mark(&mut self, call: u32) {
        self.numbers
            .iter_mut()
            .flatten()
            .for_each(|(number, marked)| {
                if *number == call {
                    *marked = true;
                }
            });
    }

    fn check_won(&mut self) -> bool {
        for i in 0..5 {
            if self.numbers[i].iter().filter(|(_, marked)| *marked).count() == 5 {
                self.won = true;
                return true;
            }
            if self
                .numbers
                .iter()
                .flatten()
                .skip(i)
                .step_by(5)
                .filter(|(_, marked)| *marked)
                .count()
                == 5
            {
                self.won = true;
                return true;
            }
        }
        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.numbers
            .iter()
            .flatten()
            .filter(|(_, marked)| !*marked)
            .map(|(num, _)| *num)
            .sum()
    }
}

fn play(calls: &[u32], mut boards: Vec<Board>, win_first: bool) -> u32 {
    let mut scores = Vec::new();
    for call in calls {
        for board in &mut boards {
            board.mark(*call);
            if !board.won && board.check_won() {
                scores.push(board.sum_unmarked() * call);
            }
        }
    }
    if win_first {
        scores[0]
    } else {
        scores[scores.len() - 1]
    }
}

fn part1(calls: &[u32], boards: &[Board]) -> u32 {
    play(calls, boards.to_vec(), true)
}

fn part2(calls: &[u32], boards: &[Board]) -> u32 {
    play(calls, boards.to_vec(), false)
}

pub fn solve(input: String) {
    let calls = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();
    let mut boards = Vec::new();
    for i in (2..input.lines().count()).step_by(6) {
        let numbers = input
            .lines()
            .skip(i)
            .take(5)
            .map(|line| {
                line.split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| (s.parse::<u32>().unwrap(), false))
                    .collect_vec()
            })
            .collect_vec();
        boards.push(Board {
            numbers,
            won: false,
        });
    }

    println!("Part 1: {}", part1(&calls, &boards));
    println!("Part 2: {}", part2(&calls, &boards));
}
