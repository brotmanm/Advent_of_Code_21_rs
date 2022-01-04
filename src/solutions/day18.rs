use Ord;

#[derive(Clone, Copy, Default, Debug)]
struct Number {
    depth: usize,
    value: u64,
}

#[derive(Clone, Debug)]
struct Pairs {
    list: Vec<Number>,
}

impl Pairs {
    fn from_string(s: &str) -> Self {
        let mut list = Vec::new();
        let mut depth: i32 = -1;
        for c in s.chars() {
            if c == '[' {
                depth += 1;
            } else if c == ']' {
                depth -= 1;
            } else if c.is_ascii_digit() {
                list.push(Number {
                    depth: depth as usize,
                    value: c.to_digit(10).unwrap() as u64,
                });
            }
        }
        Pairs { list }
    }

    fn combine(lhs: Pairs, rhs: Pairs) -> Self {
        let mut p = Pairs {
            list: lhs
                .list
                .into_iter()
                .chain(rhs.list.into_iter())
                .map(|n| Number {
                    depth: n.depth + 1,
                    ..n
                })
                .collect(),
        };
        p.reduce();
        p
    }

    fn reduce(&mut self) {
        loop {
            if self.try_explode() {
            } else if self.try_split() {
            } else {
                break;
            }
        }
    }

    fn try_explode(&mut self) -> bool {
        for i in 0..self.list.len() {
            if self.list[i].depth >= 4 {
                assert!(self.list[i].depth == 4);
                assert!(self.list[i + 1].depth == 4);
                if i > 0 {
                    self.list[i - 1].value += self.list[i].value;
                }
                if i + 2 < self.list.len() {
                    self.list[i + 2].value += self.list[i + 1].value;
                }

                let new = [Number {
                    depth: self.list[i].depth - 1,
                    value: 0,
                }];
                self.list.splice(i..(i + 2), new);
                return true;
            }
        }
        false
    }

    fn try_split(&mut self) -> bool {
        for i in 0..self.list.len() {
            if self.list[i].value >= 10 {
                let new_numbers = [
                    Number {
                        depth: self.list[i].depth + 1,
                        value: self.list[i].value / 2,
                    },
                    Number {
                        depth: self.list[i].depth + 1,
                        value: (self.list[i].value + 1) / 2,
                    },
                ];
                self.list.splice(i..(i + 1), new_numbers);
                return true;
            }
        }
        false
    }

    fn magnitude(&self) -> u64 {
        let mut numbers = self.list.clone();
        while numbers.len() > 2 {
            for i in 0..numbers.len() - 1 {
                if numbers[i].depth == numbers[i + 1].depth {
                    let new_number = [Number {
                        depth: numbers[i].depth - 1,
                        value: 3 * numbers[i].value + 2 * numbers[i + 1].value,
                    }];
                    numbers.splice(i..(i + 2), new_number);
                    break;
                }
            }
        }
        3 * numbers[0].value + 2 * numbers[1].value
    }
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| Pairs::from_string(line))
        .reduce(|accum, item| Pairs::combine(accum, item))
        .unwrap()
        .magnitude()
}

fn part2(input: &str) -> u64 {
    let all_pairs = input
        .lines()
        .map(|line| Pairs::from_string(line))
        .collect::<Vec<_>>();

    let mut max = 0;
    for i in 0..all_pairs.len() {
        for j in 0..all_pairs.len() {
            if i != j {
                let value = Pairs::combine(all_pairs[i].clone(), all_pairs[j].clone()).magnitude();
                max = Ord::max(max, value);
            }
        }
    }
    max
}

pub fn solve(input: String) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
