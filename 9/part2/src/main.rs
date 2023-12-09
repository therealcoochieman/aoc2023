use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn make_difference(line: &[i32]) -> Vec<i32> {
    let mut extrapolation = Vec::new();
    for i in 0..line.len() - 1 {
        extrapolation.push(line[i + 1] - line[i]);
    }

    extrapolation
}

fn check_all_zero(line: &[i32]) -> bool {
    for value in line {
        if value != &0 {
            return false;
        }
    }

    true
}

fn extrapolate_line(line: &str) -> i32 {
    let history = line
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<i32>>();
    let mut differences = Vec::new();
    differences.push(make_difference(&history));
    differences.insert(0, history);
    loop {
        differences.push(make_difference(differences.last().unwrap()));
        if check_all_zero(differences.last().unwrap()) {
            break;
        }
    }

    for i in (1..differences.len()).rev() {
        let bottom_value = differences[i][0];
        let top = &mut differences[i - 1];
        top.insert(0, top[0] - bottom_value);
    }

    return differences[0][0];
}

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut res = 0;
    for line in &lines {
        res += extrapolate_line(line);
    }
    println!("The sum of all extrapolated values is {}", res);
}
