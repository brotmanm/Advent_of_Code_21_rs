use itertools::Itertools;

#[derive(Debug)]
struct Program<'a> {
    input: Vec<Vec<&'a str>>,
}

impl Program<'_> {
    fn is_adder(&self, segment: &Vec<&str>) -> bool {
        if segment[4].contains('1') {
            true
        } else {
            assert!(segment[4].contains("26"));
            false
        }
    }

    fn get_int_value_from_line(&self, line: &str) -> i8 {
        line.split(' ').last().unwrap().parse::<i8>().unwrap()
    }

    fn get_adder(&self, segment: &Vec<&str>) -> i8 {
        self.get_int_value_from_line(segment[15])
    }

    fn get_subtractor(&self, segment: &Vec<&str>) -> i8 {
        self.get_int_value_from_line(segment[5])
    }

    fn get_max_values(&self, difference: i8) -> (i8, i8) {
        if difference >= 0 {
            (9 - difference, 9)
        } else {
            (9, 9 + difference)
        }
    }

    fn get_min_values(&self, difference: i8) -> (i8, i8) {
        if difference >= 0 {
            (1, 1 + difference)
        } else {
            (1 - difference, 1)
        }
    }

    fn arr_to_num(&self, arr: &[i8]) -> u64 {
        arr.iter()
            .map(|i| char::from_digit(*i as u32, 10).unwrap())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    }

    fn find_bounds(&self) -> (u64, u64) {
        let mut max_values = [0 as i8; 14];
        let mut min_values = [0 as i8; 14];

        let mut stack = Vec::new();
        for (i, segment) in self.input.iter().enumerate() {
            if self.is_adder(segment) {
                stack.push((i, self.get_adder(segment)));
            } else {
                let subtractor = self.get_subtractor(segment);
                let (j, adder) = stack.pop().unwrap();
                let max = self.get_max_values(adder + subtractor);
                let min = self.get_min_values(adder + subtractor);

                max_values[j] = max.0;
                max_values[i] = max.1;

                min_values[j] = min.0;
                min_values[i] = min.1;
            }
        }

        (self.arr_to_num(&min_values), self.arr_to_num(&max_values))
    }
}

fn part1(program: &Program) -> u64 {
    program.find_bounds().1
}

fn part2(program: &Program) -> u64 {
    program.find_bounds().0
}

pub fn solve(input: String) {
    let mut program = Vec::new();
    for i in 0..14 {
        let segment = input.lines().skip(i * 18).take(18).collect_vec();
        program.push(segment);
    }
    let program = Program { input: program };

    println!("Part 1: {}", part1(&program));
    println!("Part 2: {}", part2(&program));
}
