use std::collections::{HashMap, HashSet};

fn is_easy_digit(digit: &str) -> bool {
    digit.len() == 2 || digit.len() == 4 || digit.len() == 3 || digit.len() == 7
}

fn part1(patterns_to_outputs: &Vec<(&Vec<&str>, &Vec<&str>)>) -> u32 {
    patterns_to_outputs
        .iter()
        .fold(0, |outer_acc, pattern_to_output| {
            outer_acc
                + pattern_to_output
                    .1
                    .iter()
                    .filter(|output_value| is_easy_digit(&output_value))
                    .count() as u32
        })
}

fn str_contains_str(lhs: &str, rhs: &str) -> bool {
    lhs.chars()
        .collect::<HashSet<char>>()
        .is_superset(&rhs.chars().collect::<HashSet<char>>())
}

fn find_pattern_containing_pattern<'a>(patterns: &HashSet<&'a str>, key: &str) -> &'a str {
    for pattern in patterns {
        if str_contains_str(pattern, key) {
            return pattern;
        }
    }
    patterns.iter().next().unwrap()
}

fn find_pattern_in_pattern<'a>(patterns: &HashSet<&'a str>, key: &str) -> &'a str {
    for pattern in patterns {
        if str_contains_str(key, pattern) {
            return pattern;
        }
    }
    patterns.iter().next().unwrap()
}

fn create_mappings<'a>(
    length_map: &mut HashMap<usize, HashSet<&'a str>>,
) -> HashMap<usize, &'a str> {
    let mut pattern_map: HashMap<usize, &str> = HashMap::new();

    pattern_map.insert(1, length_map.get(&2).unwrap().iter().next().unwrap());
    pattern_map.insert(4, length_map.get(&4).unwrap().iter().next().unwrap());
    pattern_map.insert(7, length_map.get(&3).unwrap().iter().next().unwrap());
    pattern_map.insert(8, length_map.get(&7).unwrap().iter().next().unwrap());

    let three_pattern =
        find_pattern_containing_pattern(length_map.get(&5).unwrap(), pattern_map.get(&1).unwrap());
    pattern_map.insert(3, three_pattern);
    length_map.get_mut(&5).unwrap().remove(three_pattern);

    let nine_pattern =
        find_pattern_containing_pattern(length_map.get(&6).unwrap(), pattern_map.get(&3).unwrap());
    pattern_map.insert(9, nine_pattern);
    length_map.get_mut(&6).unwrap().remove(nine_pattern);

    let zero_pattern =
        find_pattern_containing_pattern(length_map.get(&6).unwrap(), pattern_map.get(&7).unwrap());
    pattern_map.insert(0, zero_pattern);
    length_map.get_mut(&6).unwrap().remove(zero_pattern);

    pattern_map.insert(6, length_map.get(&6).unwrap().iter().next().unwrap());

    let five_pattern =
        find_pattern_in_pattern(length_map.get(&5).unwrap(), pattern_map.get(&6).unwrap());
    pattern_map.insert(5, five_pattern);
    length_map.get_mut(&5).unwrap().remove(five_pattern);

    pattern_map.insert(2, length_map.get(&5).unwrap().iter().next().unwrap());

    pattern_map
}

fn patterns_match(lhs: &str, rhs: &str) -> bool {
    lhs.len() == rhs.len()
        && lhs
            .chars()
            .collect::<HashSet<char>>()
            .is_subset(&rhs.chars().collect::<HashSet<char>>())
}

fn value_from_mappings(values: &Vec<&str>, mapping: &HashMap<usize, &str>) -> u64 {
    values.iter().fold(0, |acc, cur_value| {
        acc * 10 + {
            let mut four_digit_value: i64 = -1;
            for pattern_mapping in mapping {
                if patterns_match(pattern_mapping.1, cur_value) {
                    four_digit_value = *pattern_mapping.0 as i64;
                    break;
                }
            }
            assert!(four_digit_value >= 0);
            four_digit_value as u64
        }
    })
}

fn part2(patterns_to_outputs: &Vec<(&Vec<&str>, &Vec<&str>)>) -> u64 {
    patterns_to_outputs.iter().fold(0, |acc, line| {
        acc + {
            let mut length_map = HashMap::new();
            for pattern in line.0 {
                length_map
                    .entry(pattern.len())
                    .or_insert(HashSet::new())
                    .insert(*pattern);
            }

            let mappings = create_mappings(&mut length_map);
            value_from_mappings(line.1, &mappings)
        }
    })
}

pub fn solve(input: String) {
    let inputs_to_lists = |side: usize| -> Vec<Vec<_>> {
        input
            .lines()
            .map(|line| {
                line.split("|")
                    .skip(side)
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .collect()
            })
            .collect()
    };
    let patterns = inputs_to_lists(0);
    let outputs = inputs_to_lists(1);
    let input: Vec<_> = patterns.iter().zip(outputs.iter()).collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
