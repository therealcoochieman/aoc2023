use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_line(line: &str) -> Vec<u64> {
    line.split([':', ' '])
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn possible_hold_times(time: u64, record: u64) -> u64 {
    let mut hold_times = 0;
    for speed in 0..time {
        let time_left = time - speed;
        let distance = speed * time_left;
        if distance > record {
            hold_times += 1;
        }
    }

    hold_times
}
fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let time = parse_line(&lines[0]);
    let distance = parse_line(&lines[1]);
    let mut result = 1;
    for i in 0..time.len() {
        result *= possible_hold_times(time[i], distance[i]);
    }

    println!("{}", result);
}
