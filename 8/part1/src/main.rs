use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Node {
    pub label: String,
    pub left: String,
    pub right: String,
}

impl Node {
    pub fn new(line: &str) -> (&str, Self) {
        let line: Vec<&str> = line.split('=').collect();
        let label = line[0].split(' ').next().unwrap();
        let directions = line[1]
            .split([' ', '(', ',', ')'])
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        (
            label,
            Node {
                label: label.into(),
                left: directions[0].into(),
                right: directions[1].into(),
            },
        )
    }
}

fn run_path<'a>(
    start: &'a str,
    path: &'a str,
    tree: &'a HashMap<&str, Node>,
) -> &'a str {
    let mut current = start;
    for instruction in path.chars() {
        let node = &tree[current];
        current = match instruction {
            'L' => &node.left,
            'R' => &node.right,
            _ => unreachable!(),
        };
    }

    current
}

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut tree: HashMap<&str, Node> = HashMap::new();
    let path = &lines[0];

    for line in &lines[2..] {
        let node = Node::new(line);
        tree.insert(node.0, node.1);
    }

    let mut start = "AAA";
    let mut path_runs = 0;
    while start != "ZZZ" {
        start = run_path(start, path, &tree);
        path_runs += 1;
    }
    println!("it took {} steps to reach ZZZ", path_runs * path.len());
}
