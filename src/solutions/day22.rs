use itertools::Itertools;
use Ord;

#[derive(Clone, Copy, Default, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy, Default, Debug)]
struct Rect3D {
    min: Point,
    max: Point,
}

#[derive(Clone, Copy, Default, Debug)]
struct Command {
    rect: Rect3D,
    turn_on: bool,
}

impl Rect3D {
    fn size(&self) -> u64 {
        (self.max.x - self.min.x + 1) as u64
            * (self.max.y - self.min.y + 1) as u64
            * (self.max.z - self.min.z + 1) as u64
    }

    fn is_valid(&self) -> bool {
        self.max.x >= self.min.x && self.max.y >= self.min.y && self.max.z >= self.min.z
    }

    fn intersects(&self, other: &Self) -> bool {
        self.min.x <= other.max.x
            && self.min.y <= other.max.y
            && self.min.z <= other.max.z
            && self.max.x >= other.min.x
            && self.max.y >= other.min.y
            && self.max.z >= other.min.z
    }

    fn split(&self, other: &Self) -> Vec<Rect3D> {
        assert!(self.intersects(other));
        let mut splits = Vec::new();
        let remove_rect = Rect3D {
            max: Point {
                x: Ord::min(self.max.x, other.max.x),
                y: Ord::min(self.max.y, other.max.y),
                z: Ord::min(self.max.z, other.max.z),
            },
            min: Point {
                x: Ord::max(self.min.x, other.min.x),
                y: Ord::max(self.min.y, other.min.y),
                z: Ord::max(self.min.z, other.min.z),
            },
        };

        splits.push(Rect3D {
            max: self.max,
            min: Point {
                x: remove_rect.max.x + 1,
                ..self.min
            },
        });
        splits.push(Rect3D {
            max: Point {
                x: remove_rect.min.x - 1,
                ..self.max
            },
            min: self.min,
        });
        splits.push(Rect3D {
            max: Point {
                x: remove_rect.max.x,
                ..self.max
            },
            min: Point {
                x: remove_rect.min.x,
                y: remove_rect.max.y + 1,
                ..self.min
            },
        });
        splits.push(Rect3D {
            max: Point {
                x: remove_rect.max.x,
                y: remove_rect.min.y - 1,
                ..self.max
            },
            min: Point {
                x: remove_rect.min.x,
                ..self.min
            },
        });
        splits.push(Rect3D {
            max: Point {
                x: remove_rect.max.x,
                y: remove_rect.max.y,
                ..self.max
            },
            min: Point {
                x: remove_rect.min.x,
                y: remove_rect.min.y,
                z: remove_rect.max.z + 1,
            },
        });
        splits.push(Rect3D {
            max: Point {
                x: remove_rect.max.x,
                y: remove_rect.max.y,
                z: remove_rect.min.z - 1,
            },
            min: Point {
                x: remove_rect.min.x,
                y: remove_rect.min.y,
                ..self.min
            },
        });

        splits.into_iter().filter(|r| r.is_valid()).collect_vec()
    }
}

impl Command {
    fn apply_commands(commands: &[Self]) -> u64 {
        let mut rects: Vec<Rect3D> = Vec::new();
        for command in commands {
            let mut rects_to_add: Vec<Rect3D> = Vec::new();
            rects.retain(|r| {
                if r.intersects(&command.rect) {
                    rects_to_add.append(&mut r.split(&command.rect));
                    false
                } else {
                    true
                }
            });
            rects.append(&mut rects_to_add);
            if command.turn_on {
                rects.push(command.rect);
            }
        }

        rects.iter().fold(0, |acc, r| acc + r.size())
    }
}

fn part1(commands: &[Command]) -> u64 {
    Command::apply_commands(
        &commands
            .into_iter()
            .filter(|c| {
                c.rect.min.x >= -50
                    && c.rect.min.y >= -50
                    && c.rect.min.z >= -50
                    && c.rect.max.x <= 50
                    && c.rect.max.y <= 50
                    && c.rect.max.z <= 50
            })
            .map(|c| *c)
            .collect_vec(),
    )
}

fn part2(commands: &[Command]) -> u64 {
    Command::apply_commands(commands)
}

pub fn solve(input: String) {
    let commands = input
        .lines()
        .map(|line| {
            let bounds = line
                .split(|c: char| !c.is_ascii_digit() && c != '-')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect_vec();
            let rect = Rect3D {
                min: Point {
                    x: bounds[0],
                    y: bounds[2],
                    z: bounds[4],
                },
                max: Point {
                    x: bounds[1],
                    y: bounds[3],
                    z: bounds[5],
                },
            };
            if line.contains("on") {
                Command {
                    rect,
                    turn_on: true,
                }
            } else {
                Command {
                    rect,
                    turn_on: false,
                }
            }
        })
        .collect_vec();

    println!("Part 1: {}", part1(&commands));
    println!("Part 2: {}", part2(&commands));
}
