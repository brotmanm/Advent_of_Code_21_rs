struct Enhancer {
    code: Vec<bool>,
    map: Vec<Vec<bool>>,
    empty_is_lit: bool,
}

impl Enhancer {
    fn enhance(&mut self, steps: usize) {
        for _ in 0..steps {
            let mut expanded = vec![vec![false; self.map[0].len() + 2]; self.map.len() + 2];
            for i in 0..expanded.len() {
                for j in 0..expanded[0].len() {
                    let light = self.should_light(i as i32 - 1, j as i32 - 1);
                    expanded[i][j] = light;
                }
            }
            self.map = expanded;
            self.empty_is_lit = if self.empty_is_lit {
                *self.code.last().unwrap()
            } else {
                self.code[0]
            }
        }
    }

    fn should_light(&self, row: i32, col: i32) -> bool {
        let mut index: u32 = 0;
        for i in (row - 1)..=(row + 1) {
            for j in (col - 1)..=(col + 1) {
                index <<= 1;
                if i >= 0 && i < (self.map.len() as i32) && j >= 0 && j < (self.map[0].len() as i32)
                {
                    index += self.map[i as usize][j as usize] as u32;
                } else if self.empty_is_lit {
                    index += 1;
                }
            }
        }
        self.code[index as usize]
    }

    fn count_lit(&self) -> u32 {
        self.map.iter().flatten().filter(|x| **x).count() as u32
    }
}

fn input_line_to_vec(line: &str) -> Vec<bool> {
    line.chars()
        .map(|c| {
            if c == '#' {
                true
            } else if c == '.' {
                false
            } else {
                println!("Should never get here!");
                false
            }
        })
        .collect::<Vec<_>>()
}

fn part1(code: &Vec<bool>, map: &Vec<Vec<bool>>) -> u32 {
    let mut enhancer = Enhancer {
        code: code.clone(),
        map: map.clone(),
        empty_is_lit: false,
    };
    enhancer.enhance(2);
    enhancer.count_lit()
}

fn part2(code: &Vec<bool>, map: &Vec<Vec<bool>>) -> u32 {
    let mut enhancer = Enhancer {
        code: code.clone(),
        map: map.clone(),
        empty_is_lit: false,
    };
    enhancer.enhance(50);
    enhancer.count_lit()
}

pub fn solve(input: String) {
    let mut lines = input.lines();
    let code = input_line_to_vec(lines.next().unwrap());
    lines.next();
    let mut map = Vec::new();
    for line in lines {
        map.push(input_line_to_vec(line));
    }

    println!("Part 1: {}", part1(&code, &map));
    println!("Part 2: {}", part2(&code, &map));
}
