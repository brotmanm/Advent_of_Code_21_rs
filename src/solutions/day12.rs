use std::collections::{HashMap, HashSet};

fn is_little_cave(node: &str) -> bool {
    node.chars().next().unwrap().is_lowercase()
}

fn search_from<'a>(
    graph: &HashMap<&str, Vec<&str>>,
    node: &'a str,
    mut path: HashSet<&'a str>,
    mut double_back: bool,
) -> u64 {
    if node == "end" {
        return 1;
    }

    if path.contains(node) {
        if double_back {
            double_back = false;
        } else {
            return 0;
        }
    }

    if is_little_cave(node) {
        path.insert(node);
    }
    let mut num_paths = 0;
    for adjacent in &graph[node] {
        if *adjacent != "start" {
            num_paths += search_from(graph, *adjacent, path.clone(), double_back);
        }
    }
    num_paths
}

fn part1(graph: &HashMap<&str, Vec<&str>>) -> u64 {
    let path: HashSet<&str> = HashSet::new();
    search_from(graph, "start", path, false)
}

fn part2(graph: &HashMap<&str, Vec<&str>>) -> u64 {
    let path: HashSet<&str> = HashSet::new();
    search_from(graph, "start", path, true)
}

pub fn solve(input: String) {
    let mut graph = HashMap::new();

    for edge in input
        .lines()
        .map(|line| line.split("-").collect::<Vec<_>>())
    {
        graph.entry(edge[0]).or_insert(Vec::new()).push(edge[1]);
        graph.entry(edge[1]).or_insert(Vec::new()).push(edge[0]);
    }

    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));
}
