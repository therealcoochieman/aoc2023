use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type Directions<'a> = (&'a str, &'a str);

pub fn new(line: &str) -> (&str, Directions) {
    let line: Vec<&str> = line.split('=').collect();
    let label = line[0].split(' ').next().unwrap();
    let directions = line[1]
        .split([' ', '(', ',', ')'])
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();

    (label, (directions[0], directions[1]))
}

fn run_path<'a>(
    start: &'a str,
    path: &str,
    tree: &'a HashMap<&'a str, Directions>,
) -> &'a str {
    let mut current = start;
    for instruction in path.chars() {
        let node = &tree[current];
        current = match instruction {
            'L' => &node.0,
            'R' => &node.1,
            _ => unreachable!(),
        };
    }

    current
}

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut tree: HashMap<&str, Directions> = HashMap::new();
    let path = &lines[0];
    let mut start_nodes = Vec::new();

    for line in &lines[2..] {
        let node = new(line);

        if node.0.contains('A') {
            start_nodes.push(node.0);
        }

        tree.insert(node.0, node.1);
    }

    let mut result = 1;
    for start in start_nodes {
        let mut paths = 0;
        let mut start = start;
        while !start.contains('Z') {
            start = run_path(start, path, &tree);
            paths += 1;
        }

        result *= paths;
    }

    println!(
        "It takes {} steps before being only on Z nodes",
        result * path.len()
    );
}
