use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
struct Range {
    pub start: u64,
    pub end: u64,
}

impl Range {
    pub fn is_value_in_range(&self, i: u64) -> bool {
        self.start <= i && i < self.end
    }

    pub fn does_intersect(&self, other: &Range) -> bool {
        self.is_value_in_range(other.start) || self.is_value_in_range(other.end)
    }

    pub fn intersection(&self, other: &Range) -> Range {
        Range {
            start: max(self.start, other.start),
            end: min(self.end, other.end),
        }
    }

    pub fn split_range(&self, other: &Range) -> Vec<Option<Range>> {
        let middle_range = self.intersection(other);
        let mut right_opt = None;
        if self.end < other.end {
            right_opt = Some(Range {
                start: self.end,
                end: other.end,
            })
        }
        let mut left_opt = None;
        if self.start > other.start {
            left_opt = Some(Range {
                start: other.start,
                end: self.start,
            })
        }

        [left_opt, Some(middle_range), right_opt].to_vec()
    }
}

fn build_seed_range(seeds: &[(u64, bool)]) -> Vec<(Range, bool)> {
    let mut range_seeds = Vec::new();
    for i in (0..seeds.len() - 1).step_by(2) {
        range_seeds.push((
            Range {
                start: seeds[i].0,
                end: seeds[i].0 + seeds[i + 1].0,
            },
            false,
        ))
    }

    range_seeds
}

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut line_index = 0;
    // currently seed
    let keys: Vec<(u64, bool)> = lines[line_index]
        .split([':', ' '])
        .filter_map(|s| {
            if let Ok(num) = s.parse::<u64>() {
                Some((num, false))
            } else {
                None
            }
        })
        .collect();

    let mut keys = build_seed_range(&keys);

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

        let len = keys.len();
        let mut i = 0;
        while i < len {
            if source_range.does_intersect(&keys[i].0) && !&keys[i].1 {
                let ranges = source_range.split_range(&keys[i].0);
                if let Some(left) = &ranges[0] {
                    keys.push((left.clone(), false));
                }

                if let Some(right) = &ranges[2] {
                    keys.push((right.clone(), false));
                }

                let intersection = ranges[1].as_ref().unwrap();

                let start = destination_range.start
                    + (intersection.start).abs_diff(source_range.start);
                let end = start + intersection.end.abs_diff(intersection.start);

                keys.push((Range { start, end }, true));
                keys.remove(i);
            } else {
                i += 1;
            }
        }

        line_index += 1;
    }
    println!(
        "the lowest location is {}",
        keys.iter()
            .min_by(|a, b| a.0.start.cmp(&b.0.start))
            .unwrap()
            .0
            .start
    );
}
