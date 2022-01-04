#[derive(Clone, Debug)]
enum Element {
    East,
    South,
    Empty,
}

#[derive(Clone, Debug)]
struct Layout {
    elems: Vec<Vec<Element>>,
    left_col_was_full: Vec<bool>,
    top_row_was_full: Vec<bool>,
}

impl Layout {
    fn init(elems: &Vec<Vec<Element>>) -> Self {
        Layout {
            elems: elems.clone(),
            left_col_was_full: vec![false; elems.len()],
            top_row_was_full: vec![false; elems[0].len()],
        }
    }

    fn step_east(&mut self) -> bool {
        let mut output = false;
        for r in 0..self.elems.len() {
            let mut c = 0;
            while c < self.elems[0].len() {
                let next_index = (c + 1) % self.elems[0].len();
                if let Element::East = self.elems[r][c] {
                    if let Element::Empty = self.elems[r][next_index] {
                        if c == 0 {
                            self.left_col_was_full[r] = true;
                        }
                        if c == self.elems[0].len() - 1 && self.left_col_was_full[r] {
                            c += 1;
                            continue;
                        }
                        self.elems[r][c] = Element::Empty;
                        self.elems[r][next_index] = Element::East;
                        output = true;
                        c += 1;
                    }
                }
                c += 1;
            }
        }
        output
    }

    fn step_south(&mut self) -> bool {
        let mut output = false;
        for c in 0..self.elems[0].len() {
            let mut r = 0;
            while r < self.elems.len() {
                let next_index = (r + 1) % self.elems.len();
                if let Element::South = self.elems[r][c] {
                    if let Element::Empty = self.elems[next_index][c] {
                        if r == 0 {
                            self.top_row_was_full[c] = true;
                        }
                        if r == self.elems.len() - 1 && self.top_row_was_full[c] {
                            r += 1;
                            continue;
                        }
                        self.elems[r][c] = Element::Empty;
                        self.elems[next_index][c] = Element::South;
                        output = true;
                        r += 1;
                    }
                }
                r += 1;
            }
        }
        output
    }

    fn step_until_stop(&mut self) -> u32 {
        let mut steps = 0;
        loop {
            steps += 1;
            let any_steps_east = self.step_east();
            let any_steps_south = self.step_south();
            if !any_steps_east && !any_steps_south {
                return steps;
            }
            self.left_col_was_full.iter_mut().for_each(|b| *b = false);
            self.top_row_was_full.iter_mut().for_each(|b| *b = false);
        }
    }
}

fn part1(mut layout: Layout) -> u32 {
    layout.step_until_stop()
}

pub fn solve(input: String) {
    let mut elems = Vec::new();
    for line in input.lines() {
        let mut cur_elems = Vec::new();
        for c in line.chars() {
            let element = match c {
                '>' => Element::East,
                'v' => Element::South,
                _ => Element::Empty,
            };
            cur_elems.push(element);
        }
        elems.push(cur_elems);
    }
    let layout = Layout::init(&elems);

    println!("Part 1: {}", part1(layout.clone()));
    println!("Part 2: {}", "The end... or is it?");
}
