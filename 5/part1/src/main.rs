use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn is_in_range(&self, i: u64) -> bool {
        self.start <= i && i < self.end
    }

    pub fn compute_key(&self, value: u64) -> u64 {
        value - self.start
    }

    pub fn compute_value(&self, key: u64) -> u64 {
        self.start + key
    }
}

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut line_index = 0;
    // currently seed
    let mut keys: Vec<(u64, bool)> = lines[line_index]
        .split([':', ' '])
        .filter_map(|s| {
            if let Ok(num) = s.parse::<u64>() {
                Some((num, false))
            } else {
                None
            }
        })
        .collect();

    line_index += 3;
    while line_index < lines.len() {
        let line = &lines[line_index];
        // if line is empty, we are done with a map
        if line.is_empty() {
            line_index += 2;
            keys.iter_mut().for_each(|key| key.1 = false);
            continue;
        }

        let range: Vec<u64> =
            line.split(' ').map(|s| s.parse().unwrap()).collect();

        let destination_range = Range {
            start: range[0],
            end: range[0] + range[2],
        };

        let source_range = Range {
            start: range[1],
            end: range[1] + range[2],
        };

        for key in &mut keys {
            if source_range.is_in_range(key.0) && !key.1 {
                *key = (
                    destination_range
                        .compute_value(source_range.compute_key(key.0)),
                    true,
                );
            }
        }

        line_index += 1;
    }

    println!(
        "result is {}",
        keys.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0
    )
}
